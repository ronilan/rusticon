use std::fs;
use std::path::Path;

/// Extracts data between `start_needle` and `end_needle`
fn data_from_file(text: &str, start_needle: &str, end_needle: &str) -> Vec<Option<u8>> {
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

/// Reads a "Crumbicon" file
pub fn crumbicon(file_path: &str) -> Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8), String> {
    let path = Path::new(file_path);
    let text = fs::read_to_string(path).map_err(|_| format!("File `{}` not found.", file_path))?;

    let crumbicon_data = data_from_file(&text, "<!-- crumbicon-data:", "crumbicon-data -->");
    let crumbicon_palette =
        data_from_file(&text, "<!-- crumbicon-palette:", "crumbicon-palette -->");
    let size = (crumbicon_data.len() as f64).sqrt() as u8;

    if crumbicon_data.is_empty()
        || crumbicon_palette.is_empty()
        || size as usize * size as usize != crumbicon_data.len()
    {
        return Err(format!("Invalid crumbicon file `{}`.", file_path));
    }

    Ok((crumbicon_data, crumbicon_palette, size))
}
