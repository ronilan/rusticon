use crate::cargo::{self, HtmlMetadata};
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn inject_html(html: &str, meta: &HtmlMetadata) -> String {
    let mut out = html.to_string();

    if !meta.title.is_empty() {
        if let Some(start) = out.find("<title>") {
            if let Some(rel) = out[start..].find("</title>") {
                let end = start + rel + "</title>".len();
                out.replace_range(start..end, &format!("<title>{}</title>", meta.title));
            }
        }
    }

    if !meta.min_width.is_empty() {
        out = replace_attr(&out, "data-min-width", &meta.min_width);
    }
    if !meta.min_height.is_empty() {
        out = replace_attr(&out, "data-min-height", &meta.min_height);
    }

    let mut extra = String::new();
    if !meta.description.is_empty() {
        extra.push_str(&format!(
            "<meta name=\"description\" content=\"{}\">\n",
            meta.description
        ));
    }
    if !meta.keywords.is_empty() {
        extra.push_str(&format!(
            "<meta name=\"keywords\" content=\"{}\">\n",
            meta.keywords
        ));
    }
    extra.push_str(&format!("<meta name=\"version\" content=\"{}\">\n", meta.version));

    if let Some(pos) = out.find("</head>") {
        out.insert_str(pos, &extra);
    }

    out
}

fn replace_attr(html: &str, attr: &str, value: &str) -> String {
    let needle = format!("{}=\"", attr);
    let out = html.to_string();
    let chars: Vec<char> = out.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();
    let mut i = 0;
    let mut result = String::new();
    let mut replaced = false;
    while i < chars.len() && !replaced {
        if chars[i..].starts_with(&needle_chars[..]) {
            let start = i;
            let mut j = i + needle_chars.len();
            while j < chars.len() && chars[j] != '"' {
                j += 1;
            }
            result.push_str(&chars[..start].iter().collect::<String>());
            result.push_str(&needle);
            result.push_str(value);
            result.push('"');
            result.push_str(&chars[j + 1..].iter().collect::<String>());
            replaced = true;
            break;
        }
        i += 1;
    }
    if replaced {
        result
    } else {
        out
    }
}

pub fn build() -> io::Result<()> {
    let status = Command::new("wasm-pack")
        .args(["build", "--target", "web", "--release"])
        .status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "wasm-pack build failed",
        ));
    }

    if docs_dir().exists() {
        fs::remove_dir_all(docs_dir())?;
    }
    fs::create_dir_all(docs_dir())?;

    let assets = ["style.css", "favicon.svg"];
    for asset in assets {
        let from = Path::new("web").join(asset);
        if from.exists() {
            fs::copy(&from, docs_dir().join(asset))?;
        }
    }

    let fonts_from = Path::new("web").join("fonts");
    if fonts_from.exists() {
        copy_dir_all(&fonts_from, &docs_dir().join("fonts"))?;
    }

    if Path::new("src/main.js").exists() {
        fs::copy("src/main.js", docs_main_js())?;
    }

    let pkg_from = Path::new("pkg");
    let pkg_to = docs_dir().join("pkg");
    if pkg_from.exists() {
        copy_dir_all(pkg_from, &pkg_to)?;
    }

    let meta = cargo::html_metadata();
    let index_from = match fs::read_to_string(Path::new("web").join("index.html")) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let injected = inject_html(&index_from, &meta);
    write_file(&docs_dir().join("index.html"), &injected)?;

    if !meta.cname.is_empty() {
        write_file(&docs_dir().join("CNAME"), &format!("{}\n", meta.cname))?;
    }

    println!("Wrote {}", docs_dir().display());
    println!("Web build complete.");
    Ok(())
}

fn docs_dir() -> PathBuf {
    PathBuf::from("docs")
}

fn docs_main_js() -> PathBuf {
    docs_dir().join("main.js")
}

pub fn serve() -> io::Result<()> {
    if !docs_dir().exists() {
        eprintln!("docs/ does not exist. Run: cargo run --bin package -- wasm");
        std::process::exit(1);
    }
    let listener = TcpListener::bind("0.0.0.0:4627")?;
    println!("Serving {} at http://localhost:4627", docs_dir().display());
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                let _ = handle_stream(stream);
            });
        }
    }
    Ok(())
}

fn handle_stream(mut stream: TcpStream) -> io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);
    let first_line = match reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return Ok(()),
    };

    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 || parts[0] != "GET" {
        return Ok(());
    }
    let path = parts[1].to_string();
    if path.contains("..") {
        send_status(&mut stream, 400, "Bad Request")?;
        return Ok(());
    }

    let file = if path == "/" {
        docs_dir().join("index.html")
    } else {
        let rel = path.trim_start_matches('/');
        docs_dir().join(rel)
    };

    match fs::read(&file) {
        Ok(bytes) => {
            let mime = mime_for(&file);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
                mime,
                bytes.len()
            );
            stream.write_all(response.as_bytes())?;
            stream.write_all(&bytes)?;
            Ok(())
        }
        Err(_) => send_status(&mut stream, 404, "Not Found"),
    }
}

fn send_status(stream: &mut TcpStream, code: u16, text: &str) -> io::Result<()> {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        code,
        text,
        text.len(),
        text
    );
    stream.write_all(response.as_bytes())
}

fn mime_for(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
    {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "wasm" => "application/wasm",
        "svg" => "image/svg+xml",
        "woff2" => "font/woff2",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "json" => "application/json",
        "map" => "application/json",
        _ => "application/octet-stream",
    }
}