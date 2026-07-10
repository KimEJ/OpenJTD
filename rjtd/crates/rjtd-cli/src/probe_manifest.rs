use std::fs;
use std::path::Path;

const MANIFEST_COLUMNS: usize = 19;

#[derive(Debug, Clone)]
pub(crate) struct ManifestRow {
    pub(crate) id: String,
    pub(crate) stem: String,
    pub(crate) base_id: String,
    pub(crate) priority: String,
    pub(crate) status: String,
    pub(crate) changed_variable: String,
}

pub(crate) fn read_manifest(corpus_dir: &Path) -> Result<Vec<ManifestRow>, String> {
    let manifest_path = corpus_dir.join("manifest.csv");
    let text = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("reading {}: {error}", manifest_path.display()))?;
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Err(format!("empty manifest: {}", manifest_path.display()));
    };
    let headers = header.splitn(MANIFEST_COLUMNS, ',').collect::<Vec<_>>();
    let id_index = header_index(&headers, "id")?;
    let stem_index = header_index(&headers, "filename_stem")?;
    let base_index = header_index(&headers, "base_id")?;
    let priority_index = header_index(&headers, "priority")?;
    let status_index = header_index(&headers, "status")?;
    let changed_index = header_index(&headers, "changed_variable")?;

    lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let cols = line.splitn(MANIFEST_COLUMNS, ',').collect::<Vec<_>>();
            let get = |index: usize| {
                cols.get(index)
                    .map(|value| (*value).to_string())
                    .ok_or_else(|| format!("manifest row has too few columns: {line}"))
            };
            Ok(ManifestRow {
                id: get(id_index)?,
                stem: get(stem_index)?,
                base_id: get(base_index)?,
                priority: get(priority_index)?,
                status: get(status_index)?,
                changed_variable: get(changed_index)?,
            })
        })
        .collect()
}

fn header_index(headers: &[&str], name: &str) -> Result<usize, String> {
    headers
        .iter()
        .position(|header| *header == name)
        .ok_or_else(|| format!("manifest missing required column: {name}"))
}
