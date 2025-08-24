use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};
use std::fs;
use std::path::Path;
use terminal_style::color::rgb_to_ansi8;

use std::fs::File;
use std::io::BufReader;

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

pub fn load_and_resize_image<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<[u8; 4]>>, String> {
    let path_ref = path.as_ref();

    // Open the file
    let file = File::open(path_ref).map_err(|e| format!("Failed to open file: {}", e))?;
    let buf_reader = BufReader::new(file);

    // Guess image format from the file contents
    let format = ImageFormat::from_path(path_ref)
        .map_err(|_| "File is not a recognized image format".to_string())?;

    // Load the image
    let img =
        image::load(buf_reader, format).map_err(|e| format!("Failed to load image: {}", e))?;

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

/// Reads a "Crumbicon" file, or if invalid, tries to open the provided file as a regular image.
/// 
/// # Returns
/// - `Vec<Option<u8>>`: flattened pixel data (None = transparent, Some = ANSI8 color)
/// - `Vec<Option<u8>>`: palette of unique ANSI8 colors used
/// - `u8`: size of the icon (16 for fallback images)
/// - `String`: path of the original Crumbicon file, or modified path (fallback images get `.svg`)
pub fn import_file(
    file_path: &str,
) -> Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8, String), String> {
    let path = Path::new(file_path);

    // ------------------------------
    // 1. Attempt to read as Crumbicon
    // ------------------------------
    let text = if path.exists() {
        // File exists → read content (ignore errors for now)
        fs::read_to_string(path).unwrap_or_default()
    } else {
        // File does NOT exist → return default empty 8x8 icon immediately
        let mut new_path = Path::new(file_path).to_path_buf();
        new_path.set_extension("svg"); // force `.svg` extension

        return Ok((
            vec![None; 8 * 8],          // default 8x8 empty pixels
            vec![None; 8],              // default empty palette
            8,                          // default size 8
            new_path.to_string_lossy().into_owned(), // fallback path
        ));
    };

    // -----------------------------------
    // 2. Extract crumbicon data & palette
    // -----------------------------------
    let crumbicon_data = get_crumbicon_data(&text, "<!-- crumbicon-data:", "crumbicon-data -->");
    let crumbicon_palette =
        get_crumbicon_data(&text, "<!-- crumbicon-palette:", "crumbicon-palette -->");

    // Compute size assuming square
    let size = (crumbicon_data.len() as f64).sqrt() as u8;

    // --------------------------
    // 3. Valid Crumbicon check
    // --------------------------
    if !crumbicon_data.is_empty()
        && !crumbicon_palette.is_empty()
        && size as usize * size as usize == crumbicon_data.len()
    {
        // Valid crumbicon → return original data and path
        return Ok((
            crumbicon_data,
            crumbicon_palette,
            size,
            file_path.to_string(),
        ));
    }

    // ----------------------------------------
    // 4. Fallback: try loading as regular image
    // ----------------------------------------
    match load_and_resize_image(file_path) {
        Ok(pixels_2d) => {
            let mut data = Vec::with_capacity(16 * 16); // flattened pixel array
            let mut palette = Vec::new();                // unique ANSI8 colors

            // Convert RGBA 16x16 image to Crumbicon-style pixel data
            for row in &pixels_2d {
                for &px in row {
                    let a = px[3]; // alpha channel
                    let val = if a == 0 {
                        None // transparent
                    } else {
                        let ansi8 = rgb_to_ansi8([px[0], px[1], px[2]]); // RGB → ANSI8
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
            let mut new_path = Path::new(file_path).to_path_buf();
            new_path.set_extension("svg");

            Ok((
                data,
                palette.into_iter().map(Some).collect(),
                16, // fallback size always 16
                new_path.to_string_lossy().into_owned(),
            ))
        }
        // Neither valid crumbicon nor image → error
        Err(_) => Err(format!(
            "Invalid  file: {}",
            file_path
        )),
    }
}
