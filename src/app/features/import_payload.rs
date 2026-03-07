use std::path::Path;

use crate::core::shared::ImportOutcome;

fn normalize_svg_name(file_name: &str) -> String {
    if file_name.to_lowercase().ends_with(".svg") {
        file_name.to_string()
    } else {
        format!("{}.svg", file_name)
    }
}

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

pub fn import_payload_svg(file_name: &str, text: &str) -> ImportOutcome {
    let crumbicon_data = get_crumbicon_data(text, "<!-- crumbicon-data:", "crumbicon-data -->");
    let crumbicon_palette =
        get_crumbicon_data(text, "<!-- crumbicon-palette:", "crumbicon-palette -->");

    let size = (crumbicon_data.len() as f64).sqrt() as u8;
    if !crumbicon_data.is_empty()
        && !crumbicon_palette.is_empty()
        && size as usize * size as usize == crumbicon_data.len()
    {
        return Ok((
            crumbicon_data,
            crumbicon_palette,
            size,
            normalize_svg_name(file_name),
        ));
    }

    Err(format!(
        "Invalid file: {}",
        Path::new(file_name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(file_name)
    ))
}
