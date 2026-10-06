#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn create_fontdb() -> usvg::fontdb::Database {
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    for dir in &[
        "ttfs",
        "ttfs/windows",
        "ttfs/hwp",
        "/System/Library/Fonts",
        "/System/Library/Fonts/Supplemental",
        "/Library/Fonts",
    ] {
        if std::path::Path::new(dir).exists() {
            fontdb.load_fonts_dir(dir);
        }
    }
    load_macos_mobile_asset_fonts(&mut fontdb);

    // Generic families must name an installed face. Unconditionally assigning
    // macOS names makes otherwise available Linux text disappear in usvg.
    for (generic, names) in [
        (
            usvg::fontdb::Family::Serif,
            &[
                "Hiragino Mincho ProN",
                "Noto Serif CJK JP",
                "DejaVu Serif",
                "Liberation Serif",
                "Times New Roman",
            ][..],
        ),
        (
            usvg::fontdb::Family::SansSerif,
            &[
                "Hiragino Sans",
                "Noto Sans CJK JP",
                "DejaVu Sans",
                "Liberation Sans",
                "Arial",
            ][..],
        ),
        (
            usvg::fontdb::Family::Monospace,
            &[
                "Menlo",
                "Noto Sans Mono CJK JP",
                "DejaVu Sans Mono",
                "Liberation Mono",
                "Courier New",
            ][..],
        ),
    ] {
        if let Some(name) = names.iter().find(|name| {
            fontdb
                .query(&usvg::fontdb::Query {
                    families: &[usvg::fontdb::Family::Name(name)],
                    ..Default::default()
                })
                .is_some()
        }) {
            match generic {
                usvg::fontdb::Family::Serif => fontdb.set_serif_family(*name),
                usvg::fontdb::Family::SansSerif => fontdb.set_sans_serif_family(*name),
                usvg::fontdb::Family::Monospace => fontdb.set_monospace_family(*name),
                _ => unreachable!(),
            }
        }
    }
    fontdb
}

#[cfg(not(target_arch = "wasm32"))]
fn load_macos_mobile_asset_fonts(fontdb: &mut usvg::fontdb::Database) {
    let base = std::path::Path::new("/System/Library/AssetsV2");
    let Ok(entries) = std::fs::read_dir(base) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name.starts_with("com_apple_MobileAsset_Font") {
            load_font_dirs_recursive(fontdb, &path, 0);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_font_dirs_recursive(
    fontdb: &mut usvg::fontdb::Database,
    path: &std::path::Path,
    depth: usize,
) {
    if depth > 4 {
        return;
    }
    fontdb.load_fonts_dir(path);

    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            load_font_dirs_recursive(fontdb, &path, depth + 1);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn add_font_fallbacks(svg: &str) -> String {
    svg.replace(
        "font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, Noto Sans CJK JP, sans-serif\"",
        "font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Hiragino Sans GB, Yu Gothic, Meiryo, Apple SD Gothic Neo, Noto Sans CJK JP, sans-serif\"",
    )
}
