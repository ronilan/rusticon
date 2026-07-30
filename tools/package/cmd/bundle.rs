use crate::cargo;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn macos() {
    let app_name = cargo::bundle_app_name();
    let bin_name = cargo::bundle_binary_name();
    let icon_source = cargo::bundle_icon_source();

    println!("Bundling \"{}\"", app_name);

    let native_dir = Path::new("dist/native");
    let app_bundle = native_dir.join(format!("{}.app", app_name));
    let contents_dir = app_bundle.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let resources_dir = contents_dir.join("Resources");

    // 1. Create directory structure
    fs::create_dir_all(&macos_dir).expect("Failed to create MacOS directory");
    fs::create_dir_all(&resources_dir).expect("Failed to create Resources directory");

    // 2. Copy binary
    let binary_path = native_dir.join(&bin_name);
    if binary_path.exists() {
        fs::copy(&binary_path, macos_dir.join(&bin_name)).expect("Failed to copy binary");
    } else {
        eprintln!(
            "Error: Binary {:?} not found. Run build first.",
            binary_path
        );
        std::process::exit(1);
    }

    // 3. Create icon
    let icon_path = Path::new(&icon_source);
    if icon_path.exists() {
        println!("Creating icon with native macOS margins");
        let iconset_dir = native_dir.join("AppIcon.iconset");
        if !iconset_dir.exists() {
            fs::create_dir_all(&iconset_dir).expect("Failed to create iconset dir");
        }

        let sizes = [16usize, 32, 64, 128, 256, 512, 1024];
        for &size in &sizes {
            let png_path = iconset_dir.join(format!("icon_{}x{}.png", size, size));

            let inner_size = (size as f64 * 0.8) as u32;
            let _ = Command::new("rsvg-convert")
                .args([
                    "-w",
                    &inner_size.to_string(),
                    "-h",
                    &inner_size.to_string(),
                    &icon_source,
                ])
                .stdout(fs::File::create(&png_path).expect("Failed to create PNG"))
                .status();

            let _ = Command::new("sips")
                .args([
                    "-p",
                    &size.to_string(),
                    &size.to_string(),
                    &png_path.to_string_lossy(),
                    "--out",
                    &png_path.to_string_lossy(),
                ])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();

            if size <= 512 {
                let inner_size_2x = (size as f64 * 2.0 * 0.8) as u32;
                let png_path_2x = iconset_dir.join(format!("icon_{}x{}@2x.png", size, size));

                let _ = Command::new("rsvg-convert")
                    .args([
                        "-w",
                        &inner_size_2x.to_string(),
                        "-h",
                        &inner_size_2x.to_string(),
                        &icon_source,
                    ])
                    .stdout(fs::File::create(&png_path_2x).expect("Failed to create PNG"))
                    .status();

                let _ = Command::new("sips")
                    .args([
                        "-p",
                        &(size * 2).to_string(),
                        &(size * 2).to_string(),
                        &png_path_2x.to_string_lossy(),
                        "--out",
                        &png_path_2x.to_string_lossy(),
                    ])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();
            }
        }

        // Convert iconset to icns
        if iconset_dir.join("icon_1024x1024.png").exists() {
            let icns_path = resources_dir.join("AppIcon.icns");
            let _ = Command::new("iconutil")
                .args([
                    "-c",
                    "icns",
                    &iconset_dir.to_string_lossy(),
                    "-o",
                    &icns_path.to_string_lossy(),
                ])
                .status();
            let _ = fs::remove_dir_all(&iconset_dir);
        }
    }

    // 4. Copy Info.plist
    if Path::new("Info.plist").exists() {
        fs::copy("Info.plist", contents_dir.join("Info.plist")).expect("Failed to copy Info.plist");
    }

    println!("Successfully created {:?}", app_bundle);

    // 5. Create DMG
    let staging_dir = native_dir.join("dmg");
    let _ = fs::remove_dir_all(&staging_dir);
    fs::create_dir_all(&staging_dir).expect("Failed to create dmg staging dir");

    let staged_app = staging_dir.join(format!("{}.app", app_name));
    let _ = fs::remove_dir_all(&staged_app);
    copy_dir_recursive(&app_bundle, &staged_app).expect("Failed to copy app bundle for DMG");

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("/Applications", staging_dir.join("Applications")).ok();
    }

    let dmg_path = native_dir.join(format!("{}.dmg", app_name));
    let _ = fs::remove_file(&dmg_path);

    let _ = Command::new("hdiutil")
        .stdout(Stdio::null())
        .args([
            "create",
            "-volname",
            &app_name,
            "-srcfolder",
            &staging_dir.to_string_lossy(),
            "-ov",
            "-format",
            "UDZO",
            &dmg_path.to_string_lossy(),
        ])
        .status();

    let _ = fs::remove_dir_all(&staging_dir);
    println!("Created {:?}", dmg_path);
}
#[cfg(target_os = "windows")]
pub fn windows() {
    let app_name = cargo::bundle_app_name();
    let icon_source = cargo::bundle_icon_source();
    let exe_path = Path::new("dist/native").join(format!("{}.exe", app_name));

    println!("Bundling Windows version for: {}", app_name);

    if !exe_path.exists() {
        eprintln!("Error: {:?} not found. Run build first.", exe_path);
        std::process::exit(1);
    }

    let icon_path = Path::new(&icon_source);
    if icon_path.exists() {
        println!("Generating .ico from {}...", icon_source);

        // 1. Render SVG to PNG (256x256) using resvg (pure Rust, no external tools)
        let svg_data = fs::read(&icon_source).expect("Failed to read SVG file");
        let rtree = resvg::usvg::Tree::from_data(&svg_data, &resvg::usvg::Options::default())
            .expect("Failed to parse SVG");

        // Fit the SVG (512x512) to 256x256 by computing a scale transform
        let svg_size = rtree.size();
        let scale = 256.0 / svg_size.width().max(svg_size.height());
        let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);

        let mut pixmap = resvg::tiny_skia::Pixmap::new(256, 256).expect("Failed to create pixmap");
        resvg::render(&rtree, transform, &mut pixmap.as_mut());
        let png_data = pixmap.encode_png().expect("Failed to encode PNG");

        // 2. Convert PNG to ICO format
        let ico_data = png_to_ico(&png_data);

        // 3. Inject ICO into EXE using Windows native APIs
        println!("Injecting icon into {}...", exe_path.display());

        inject_ico_into_exe(&exe_path, &ico_data).expect("Failed to inject icon into executable");
    }

    println!("Successfully bundled Windows version for: {}", app_name);
}

#[cfg(not(target_os = "windows"))]
pub fn windows() {
    eprintln!("Error: Windows bundling is only supported on Windows.");
    std::process::exit(1);
}

/// Converts a 256x256 RGBA PNG buffer into an ICO file.
#[cfg(target_os = "windows")]
fn png_to_ico(png_data: &[u8]) -> Vec<u8> {
    // ICO header: reserved(2)=0, type(2)=1 (icon), count(2)=1
    let mut ico = Vec::new();
    ico.extend_from_slice(&[0u8, 0, 1, 0, 1, 0]);

    // ICO directory entry for a 256x256 image:
    // width(1)=0 (means 256), height(1)=0 (means 256), colors(1)=0,
    // reserved(1)=0, planes(2)=1, bpp(2)=32, size(4), offset(4)=22
    let size = png_data.len() as u32;
    let offset: u32 = 6 + 16; // header + 1 directory entry
    ico.extend_from_slice(&[0u8, 0, 0, 0, 1, 0, 32, 0]);
    ico.extend_from_slice(&size.to_le_bytes());
    ico.extend_from_slice(&offset.to_le_bytes());

    // Append PNG data
    ico.extend_from_slice(png_data);

    ico
}

/// Injects ICO data into a Windows PE executable as icon resources.
#[cfg(target_os = "windows")]
fn inject_ico_into_exe(exe_path: &Path, ico_data: &[u8]) -> std::io::Result<()> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::System::LibraryLoader::*;

    // Convert path to wide string
    let wide_path: Vec<u16> = OsStr::new(exe_path.as_os_str())
        .encode_wide()
        .chain(once(0))
        .collect();

    // RT_ICON = 3, RT_GROUP_ICON = 14
    const RT_ICON: u16 = 3;
    const RT_GROUP_ICON: u16 = 14;
    const LANG_EN_US: u16 = 1033;

    // Begin update resource
    let handle = unsafe { BeginUpdateResourceW(wide_path.as_ptr(), FALSE) };
    if handle.is_null() {
        return Err(std::io::Error::last_os_error());
    }

    // Parse the ICO to extract individual icon images and build group icon entry
    let count = u16::from_le_bytes([ico_data[4], ico_data[5]]) as usize;

    // GRPICONDIR has the same structure as ICONDIR but with nID instead of dwOffset
    let mut group_icon = Vec::new();
    group_icon.extend_from_slice(&ico_data[0..6]); // header (reserved, type, count)

    for i in 0..count {
        let entry_offset = 6 + i * 16;
        if entry_offset + 16 > ico_data.len() {
            break;
        }

        // Read directory entry
        let width = ico_data[entry_offset];
        let height = ico_data[entry_offset + 1];
        let color_count = ico_data[entry_offset + 2];
        let _reserved = ico_data[entry_offset + 3];
        let planes = u16::from_le_bytes([ico_data[entry_offset + 4], ico_data[entry_offset + 5]]);
        let bpp = u16::from_le_bytes([ico_data[entry_offset + 6], ico_data[entry_offset + 7]]);
        let image_size = u32::from_le_bytes([
            ico_data[entry_offset + 8],
            ico_data[entry_offset + 9],
            ico_data[entry_offset + 10],
            ico_data[entry_offset + 11],
        ]);
        let image_offset = u32::from_le_bytes([
            ico_data[entry_offset + 12],
            ico_data[entry_offset + 13],
            ico_data[entry_offset + 14],
            ico_data[entry_offset + 15],
        ]);

        let image_data = &ico_data[image_offset as usize..][..image_size as usize];

        // Resource ID for this icon (1-based)
        let icon_id = (i + 1) as u16;

        // Add RT_ICON resource
        let result = unsafe {
            UpdateResourceW(
                handle,
                RT_ICON as usize as *const u16,
                icon_id as usize as *const u16,
                LANG_EN_US,
                image_data.as_ptr() as *const std::ffi::c_void,
                image_data.len() as u32,
            )
        };
        if result == FALSE {
            unsafe { EndUpdateResourceW(handle, TRUE) };
            return Err(std::io::Error::last_os_error());
        }

        // Build GRPICONDIRENTRY (same as ICONDIRENTRY but nID replaces dwOffset)
        group_icon.push(width);
        group_icon.push(height);
        group_icon.push(color_count);
        group_icon.push(_reserved);
        group_icon.extend_from_slice(&planes.to_le_bytes());
        group_icon.extend_from_slice(&bpp.to_le_bytes());
        group_icon.extend_from_slice(&image_size.to_le_bytes());
        group_icon.extend_from_slice(&icon_id.to_le_bytes()); // nID instead of offset
    }

    // Add RT_GROUP_ICON resource with name=1, language=1033
    let result = unsafe {
        UpdateResourceW(
            handle,
            RT_GROUP_ICON as usize as *const u16,
            1usize as *const u16,
            LANG_EN_US,
            group_icon.as_ptr() as *const std::ffi::c_void,
            group_icon.len() as u32,
        )
    };
    if result == FALSE {
        unsafe { EndUpdateResourceW(handle, TRUE) };
        return Err(std::io::Error::last_os_error());
    }

    // Commit changes
    let result = unsafe { EndUpdateResourceW(handle, FALSE) };
    if result == FALSE {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if entry_type.is_dir() {
                copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
    }
    Ok(())
}
