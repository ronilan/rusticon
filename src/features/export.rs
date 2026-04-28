use little_tui::*;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

fn to_colon_list(values: &[Option<u8>]) -> String {
    values
        .iter()
        .map(|v| match v {
            Some(n) => n.to_string(),
            None => "void".to_string(),
        })
        .collect::<Vec<_>>()
        .join(":")
}

pub fn build_svg(
    data: &[Option<u8>],
    palette: &[Option<u8>],
    rows: usize,
    cols: usize,
    px: usize,
) -> String {
    let width = cols * px;
    let height = rows * px;

    let mut out = String::with_capacity(4096);
    out.push_str(&format!(
        "<svg version=\"1.1\" baseProfile=\"full\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        width, height
    ));

    for (i, cell) in data.iter().enumerate() {
        let x = (i % cols) * px;
        let y = (i / cols) * px;

        let fill = match cell {
            Some(idx) => Colors::ansi8_to_hex(*idx),
            None => "none".to_string(),
        };

        out.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
            x, y, px, px, fill
        ));
    }

    out.push_str("\n</svg>\n");

    let data_str = to_colon_list(data);
    let palette_str = to_colon_list(palette);
    out.push_str(&format!(
        "<!-- crumbicon-data:{}crumbicon-data --><!-- crumbicon-palette:{}crumbicon-palette -->\n",
        data_str, palette_str
    ));

    out
}

/// Export a Crumbicon SVG to a file (native target only).
#[cfg(not(target_arch = "wasm32"))]
pub fn export_svg<P: AsRef<Path>>(
    data: &[Option<u8>],
    palette: &[Option<u8>],
    rows: usize,
    cols: usize,
    px: usize,
    target: P,
) -> Result<(), String> {
    let out = build_svg(data, palette, rows, cols, px);
    fs::write(target, out).map_err(|e| format!("Failed to write SVG: {}", e))?;
    Ok(())
}
