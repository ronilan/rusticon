use std::fs;

fn extract_section(text: &str, header: &str) -> String {
    let marker = format!("[{}]", header);
    if let Some(idx) = text.find(&marker) {
        let after = &text[idx + marker.len()..];
        let end = after
            .find('\n')
            .map(|nl| {
                let rest = &after[nl..];
                rest.find("\n[").map(|i| nl + 1 + i).unwrap_or(after.len())
            })
            .unwrap_or(after.len());
        after[..end].trim().to_string()
    } else {
        String::new()
    }
}

fn get_field(section: &str, key: &str) -> String {
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(key) {
            let rest = &trimmed[key.len()..].trim_start();
            if rest.starts_with('=') {
                let val = rest[1..].trim();
                if val.starts_with('"') && val.ends_with('"') && val.len() >= 2 {
                    return val[1..val.len() - 1].to_string();
                }
            }
        }
    }
    String::new()
}

pub fn get_package_name() -> String {
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap_or_default();
    let pkg_section = extract_section(&cargo_toml, "package");
    let name = get_field(&pkg_section, "name");
    if name.is_empty() {
        "incredible_app_template".to_string()
    } else {
        name
    }
}
