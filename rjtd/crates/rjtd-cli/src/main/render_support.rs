use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExportOptions {
    pub(crate) format: String,
    pub(crate) output: Option<String>,
}

pub(crate) fn export_options(args: impl Iterator<Item = String>) -> Result<ExportOptions, String> {
    let mut format = None;
    let mut output = None;
    let mut args = args.peekable();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--format" => {
                let Some(value) = args.next() else {
                    return Err("missing value for `--format`".to_string());
                };
                format = Some(value);
            }
            "--output" | "-o" => {
                let Some(value) = args.next() else {
                    return Err(format!("missing value for `{arg}`"));
                };
                output = Some(value);
            }
            other => {
                return Err(format!(
                    "unexpected export argument `{other}`; usage: rjtd export <file> --format <json|md|text|html|pdf> [-o output.pdf]"
                ));
            }
        }
    }

    Ok(ExportOptions {
        format: format.ok_or_else(|| {
            "usage: rjtd export <file> --format <json|md|text|html|pdf> [-o output.pdf]".to_string()
        })?,
        output,
    })
}

pub(crate) fn write_file(path: impl AsRef<Path>, bytes: &[u8]) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create `{}`: {error}", parent.display()))?;
    }
    let temp_path = temporary_output_path(path)?;
    let write_result = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)
        .and_then(|mut file| file.write_all(bytes));

    if let Err(error) = write_result {
        let _ = std::fs::remove_file(&temp_path);
        return Err(format!("cannot write `{}`: {error}", path.display()));
    }

    if let Err(error) = std::fs::rename(&temp_path, path) {
        let _ = std::fs::remove_file(&temp_path);
        return Err(format!("cannot write `{}`: {error}", path.display()));
    }

    Ok(())
}

pub(crate) fn temporary_output_path(path: &Path) -> Result<std::path::PathBuf, String> {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("invalid output path `{}`", path.display()))?;
    let process_id = std::process::id();
    for attempt in 0..1000 {
        let candidate = parent.join(format!(".{file_name}.{process_id}.{attempt}.tmp"));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(format!(
        "cannot choose temporary output path for `{}`",
        path.display()
    ))
}
