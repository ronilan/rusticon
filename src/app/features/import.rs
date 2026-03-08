use image::{imageops::FilterType, DynamicImage, GenericImageView};
use little_tui::Colors;
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;

/// Extracts data between `start_needle` and `end_needle`
fn get_crumbicon_data(text: &str, start_needle: &str, end_needle: &str) -> Vec<Option<u8>> {
    let start_pos = match text.find(start_needle) {
        Some(pos) => pos + start_needle.len(),
        None => return vec![],
    };
    let end_pos = match text.find(end_needle) {
        Some(pos) => pos,
        None => return vec![],
    };

    text[start_pos..end_pos]
        .split(':')
        .map(|item| {
            let item = item.trim();
            if item.is_empty() || item == "void" {
                None
            } else {
                item.parse::<u8>().ok()
            }
        })
        .collect()
}

pub fn load_and_resize_image_bytes(bytes: &[u8]) -> Result<Vec<Vec<[u8; 4]>>, String> {
    // Guess image format from bytes first; fall back to the image crate detector.
    let img = match image::guess_format(bytes) {
        Ok(format) => image::load_from_memory_with_format(bytes, format)
            .map_err(|e| format!("Failed to load image: {}", e))?,
        Err(_) => {
            image::load_from_memory(bytes).map_err(|e| format!("Failed to load image: {}", e))?
        }
    };

    // Resize to 16x16 (forcing size)
    let resized: DynamicImage = img.resize_exact(16, 16, FilterType::Nearest);

    // Convert to 2D vector of RGBA
    let mut pixels_2d = vec![vec![[0u8; 4]; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            pixels_2d[y][x] = resized
                .get_pixel(x.try_into().unwrap(), y.try_into().unwrap())
                .0;
        }
    }

    Ok(pixels_2d)
}

/// Parses bytes as a Crumbicon payload, or if invalid, tries as a regular image.
///
/// # Returns
/// - `Vec<Option<u8>>`: flattened pixel data (None = transparent, Some = ANSI8 color)
/// - `Vec<Option<u8>>`: palette of unique ANSI8 colors used
/// - `u8`: size of the icon (16 for fallback images)
/// - `String`: path/name of the original Crumbicon file, or modified path (`.svg`) for images.
pub fn import_bytes(
    file_name: &str,
    bytes: &[u8],
) -> Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8, String), String> {
    // Attempt to read as Crumbicon
    let text = String::from_utf8_lossy(bytes).to_string();

    // Extract crumbicon data & palette
    let crumbicon_data = get_crumbicon_data(&text, "<!-- crumbicon-data:", "crumbicon-data -->");
    let crumbicon_palette =
        get_crumbicon_data(&text, "<!-- crumbicon-palette:", "crumbicon-palette -->");

    // Compute size assuming square
    let size = (crumbicon_data.len() as f64).sqrt() as u8;

    // Valid Crumbicon check
    if !crumbicon_data.is_empty()
        && !crumbicon_palette.is_empty()
        && size as usize * size as usize == crumbicon_data.len()
    {
        // Valid crumbicon → return original data and path
        return Ok((
            crumbicon_data,
            crumbicon_palette,
            size,
            file_name.to_string(),
        ));
    }

    // Try loading as regular image
    match load_and_resize_image_bytes(bytes) {
        Ok(pixels_2d) => {
            let mut data = Vec::with_capacity(16 * 16); // flattened pixel array
            let mut palette = Vec::new(); // unique ANSI8 colors

            // Convert RGBA 16x16 image to Crumbicon-style pixel data
            for row in &pixels_2d {
                for &px in row {
                    let a = px[3]; // alpha channel
                    let val = if a == 0 {
                        None // transparent
                    } else {
                        let ansi8 = Colors::rgb_to_ansi8([px[0], px[1], px[2]]); // RGB → ANSI8
                        Some(ansi8)
                    };
                    data.push(val);

                    // Track unique palette colors
                    if let Some(v) = val {
                        if !palette.contains(&v) {
                            palette.push(v);
                        }
                    }
                }
            }

            // For fallback images, overwrite path with `.svg`
            let mut new_path = Path::new(file_name).to_path_buf();
            new_path.set_extension("svg");

            Ok((
                data,
                palette.into_iter().map(Some).collect(),
                16, // fallback size always 16
                new_path.to_string_lossy().into_owned(),
            ))
        }
        // Neither valid crumbicon nor image → error
        Err(_) => Err(format!("Invalid file: {}", file_name)),
    }
}

/// Reads a "Crumbicon" file, or if invalid, tries to open the provided file as a regular image.
///
/// Native-only path-based importer used by terminal mode.
#[cfg(not(target_arch = "wasm32"))]
pub fn import_file(
    file_path: &str,
) -> Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8, String), String> {
    let path = Path::new(file_path);

    if !path.exists() {
        let mut new_path = Path::new(file_path).to_path_buf();
        new_path.set_extension("svg");

        if new_path.exists() {
            return import_file(&new_path.to_string_lossy());
        }

        return Ok((
            vec![None; 8 * 8],
            vec![None; 8],
            8,
            new_path.to_string_lossy().into_owned(),
        ));
    }

    let bytes = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
    import_bytes(file_path, &bytes)
}
