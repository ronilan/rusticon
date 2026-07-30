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
            let rest = &trimmed[key.len()..];
            let rest = rest.trim_start();
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

pub struct CurrentValues {
    pub name: String,
    pub app_name: String,
    pub tagline: String,
    pub keywords: String,
    pub description: String,
}

pub fn read_current_values() -> CurrentValues {
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap_or_default();

    let pkg_section = extract_section(&cargo_toml, "package");
    let html_section = extract_section(&cargo_toml, "package.metadata.html");
    let bundle_section = extract_section(&cargo_toml, "package.metadata.bundle");

    let cargo_name = get_field(&pkg_section, "name");
    let app_name_raw = get_field(&bundle_section, "app_name");
    let description = get_field(&html_section, "description");
    let keywords = get_field(&html_section, "keywords");

    let title = get_field(&html_section, "title");
    let app_name = if app_name_raw.is_empty() {
        cargo_name.clone()
    } else {
        app_name_raw
    };

    let tagline = if !app_name.is_empty() {
        let prefix = format!("{} :: ", app_name);
        if title.starts_with(&prefix) {
            title[prefix.len()..].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    CurrentValues {
        name: cargo_name.clone(),
        app_name,
        tagline,
        keywords,
        description,
    }
}
