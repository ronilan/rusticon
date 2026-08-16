use std::fs;
use std::path::Path;

fn replace_field_value(line: &str, key: &str, value: &str) -> String {
    let leading_len = line.len() - line.trim_start().len();
    let leading = &line[..leading_len];
    let trimmed = line.trim_start();
    if trimmed.starts_with(key) {
        let after_key = &trimmed[key.len()..];
        if after_key.trim_start().starts_with('=') {
            if let Some(eq_pos) = after_key.find('=') {
                let before_eq = &after_key[..eq_pos];
                let after_eq = &after_key[eq_pos + 1..];
                if let Some(quote_pos) = after_eq.find('"') {
                    let before_value = &after_eq[..quote_pos];
                    return format!(
                        "{}{}{}={}\"{}\"",
                        leading,
                        key,
                        before_eq,
                        before_value,
                        value
                    );
                }
            }
            return format!("{}{}= \"{}\"", leading, key, value);
        }
    }
    line.to_string()
}

fn transform_cargo_toml(
    content: &str,
    name: &str,
    app_name: &str,
    description: Option<&str>,
    keywords: Option<&str>,
    display_title: &str,
) -> String {
    let mut in_package = false;
    let mut in_metadata_html = false;
    let mut in_metadata_bundle = false;
    let mut in_bin = false;
    let mut pending_bin_name_idx: Option<usize> = None;
    let mut current_bin_path = String::new();

    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim().to_string();

        if trimmed == "[package]" {
            in_package = true;
            in_metadata_html = false;
            in_metadata_bundle = false;
            in_bin = false;
            pending_bin_name_idx = None;
        } else if trimmed == "[package.metadata.html]" {
            in_package = false;
            in_metadata_html = true;
            in_metadata_bundle = false;
            in_bin = false;
            pending_bin_name_idx = None;
        } else if trimmed == "[package.metadata.bundle]" {
            in_package = false;
            in_metadata_html = false;
            in_metadata_bundle = true;
            in_bin = false;
            pending_bin_name_idx = None;
        } else if trimmed == "[[bin]]" {
            in_package = false;
            in_metadata_html = false;
            in_metadata_bundle = false;
            in_bin = true;
            current_bin_path = String::new();
            pending_bin_name_idx = None;
        } else if trimmed.starts_with('[') {
            in_package = false;
            in_metadata_html = false;
            in_metadata_bundle = false;
            in_bin = false;
            pending_bin_name_idx = None;
        }

        if in_package {
            let ts = lines[i].trim_start().to_string();
            if ts.starts_with("name ") || ts.starts_with("name=") {
                lines[i] = replace_field_value(&lines[i], "name", name);
            } else if ts.starts_with("default-run") {
                lines[i] = replace_field_value(&lines[i], "default-run", name);
            }
        }
        if in_metadata_html {
            let ts = lines[i].trim_start().to_string();
            if ts.starts_with("title") {
                lines[i] = replace_field_value(&lines[i], "title", display_title);
            } else if ts.starts_with("description") {
                if let Some(desc) = description {
                    lines[i] = replace_field_value(&lines[i], "description", desc);
                }
            } else if ts.starts_with("keywords") {
                if let Some(kw) = keywords {
                    lines[i] = replace_field_value(&lines[i], "keywords", kw);
                }
            }
        }
        if in_metadata_bundle {
            let ts = lines[i].trim_start().to_string();
            if ts.starts_with("app_name") {
                lines[i] = replace_field_value(&lines[i], "app_name", app_name);
            } else if ts.starts_with("binary_name") {
                lines[i] =
                    replace_field_value(&lines[i], "binary_name", &format!("{}_macos", name));
            }
        }
        if in_bin {
            let ts = lines[i].trim_start().to_string();
            if ts.starts_with("path") {
                if let Some(start) = lines[i].find('"') {
                    if let Some(end) = lines[i][start + 1..].find('"') {
                        current_bin_path = lines[i][start + 1..start + 1 + end].to_string();
                    }
                }
            } else if ts.starts_with("name ") || ts.starts_with("name=") {
                pending_bin_name_idx = Some(i);
            }

            if let Some(idx) = pending_bin_name_idx {
                if !current_bin_path.is_empty() {
                    let new_bin_name = if current_bin_path.contains("macos.rs") {
                        Some(format!("{}_macos", name))
                    } else if current_bin_path.contains("windows.rs") {
                        Some(format!("{}_windows", name))
                    } else if current_bin_path.contains("src/main.rs") {
                        Some(name.to_string())
                    } else {
                        None
                    };
                    if let Some(bn) = new_bin_name {
                        lines[idx] = replace_field_value(&lines[idx], "name", &bn);
                    }
                    pending_bin_name_idx = None;
                }
            }
        }

        i += 1;
    }
    lines.join("\n")
}

fn transform_info_plist(content: &str, name: &str, app_name: &str) -> String {
    let mut r = content.to_string();
    if let Some(start) = r.find("<key>CFBundleExecutable</key>") {
        if let Some(s) = r[start..].find("<string>") {
            let abs_s = start + s + 8;
            if let Some(e) = r[abs_s..].find("</string>") {
                r = format!("{}{}_macos{}", &r[..abs_s], name, &r[abs_s + e..]);
            }
        }
    }
    if let Some(start) = r.find("<key>CFBundleName</key>") {
        if let Some(s) = r[start..].find("<string>") {
            let abs_s = start + s + 8;
            if let Some(e) = r[abs_s..].find("</string>") {
                r = format!("{}{}{}", &r[..abs_s], app_name, &r[abs_s + e..]);
            }
        }
    }
    r
}

fn transform_package_json(content: &str, npm_name: &str) -> String {
    if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(obj) = v.as_object_mut() {
            obj.insert(
                "name".to_string(),
                serde_json::Value::String(npm_name.to_string()),
            );
        }
        if let Ok(s) = serde_json::to_string_pretty(&v) {
            return s + "\n";
        }
    }
    content.to_string()
}

fn transform_main_ts(content: &str, name: &str) -> String {
    let mut result = String::new();
    for line in content.lines() {
        if line.starts_with("import init, { main } from \"../pkg/") {
            result.push_str(&format!(
                "import init, {{ main }} from \"../pkg/{}.js\";\n",
                name
            ));
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

fn transform_html(
    content: &str,
    display_title: &str,
    description: Option<&str>,
    keywords: Option<&str>,
) -> String {
    let mut r = content.to_string();
    if let Some(s) = r.find("<title>") {
        if let Some(e) = r[s..].find("</title>") {
            r = format!(
                "{}<title>{}</title>{}",
                &r[..s],
                display_title,
                &r[s + e + 8..]
            );
        }
    }
    if let Some(desc) = description {
        let needle = "name=\"description\" content=\"";
        if let Some(s) = r.find(needle) {
            let start = s + needle.len();
            if let Some(e) = r[start..].find('"') {
                r = format!("{}{}{}", &r[..start], desc, &r[start + e..]);
            }
        }
    }
    if let Some(kw) = keywords {
        let needle = "name=\"keywords\" content=\"";
        if let Some(s) = r.find(needle) {
            let start = s + needle.len();
            if let Some(e) = r[start..].find('"') {
                r = format!("{}{}{}", &r[..start], kw, &r[start + e..]);
            }
        }
    }
    r
}

pub fn apply_changes(
    name: &str,
    app_name: &str,
    npm_name: &str,
    tagline: &str,
    keywords: &str,
    description: &str,
) {
    let display_title = if !tagline.is_empty() {
        format!("{} :: {}", app_name, tagline)
    } else {
        app_name.to_string()
    };
    let desc_opt = if description.is_empty() {
        None
    } else {
        Some(description)
    };
    let kw_opt = if keywords.is_empty() {
        None
    } else {
        Some(keywords)
    };

    let files = [
        "Cargo.toml",
        "Info.plist",
        "package.json",
        "src/main.ts",
        "docs/index.html",
        "web/index.html",
    ];

    let mut updated = 0usize;
    for file in &files {
        let path = Path::new(file);
        if !path.exists() {
            eprintln!("Skip missing: {}", file);
            continue;
        }
        let original = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {}: {}", file, e);
                continue;
            }
        };
        let transformed = match *file {
            "Cargo.toml" => {
                transform_cargo_toml(&original, name, app_name, desc_opt, kw_opt, &display_title)
            }
            "Info.plist" => transform_info_plist(&original, name, app_name),
            "package.json" => transform_package_json(&original, npm_name),
            "src/main.ts" => transform_main_ts(&original, name),
            _ => transform_html(&original, &display_title, desc_opt, kw_opt),
        };
        if transformed != original {
            match fs::write(path, &transformed) {
                Ok(_) => {
                    println!("Updated: {}", file);
                    updated += 1;
                }
                Err(e) => eprintln!("Error writing {}: {}", file, e),
            }
        } else {
            println!("Unchanged: {}", file);
        }
    }
    println!("\nDone! {} file(s) updated.", updated);
}
