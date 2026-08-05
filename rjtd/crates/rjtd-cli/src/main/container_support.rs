use rjtd_core::container::CfbSectorChain;

use super::support::write_stdout_line;

pub(crate) fn print_entry_size(
    entries: &[rjtd_core::container::ContainerEntry],
    path: &str,
    label: &str,
) -> Result<(), String> {
    let value = entries
        .iter()
        .find(|entry| entry.path() == path)
        .map(|entry| entry.size().to_string())
        .unwrap_or_else(|| "-".to_string());
    write_stdout_line(&format!("{label}\t{value}"))
}

pub(crate) fn write_cfb_chain(label: &str, chain: &CfbSectorChain) -> Result<(), String> {
    write_stdout_line(&format!(
        "{}\t{}\t{}\t{}",
        label,
        chain.status().as_str(),
        chain.sectors().len(),
        format_sector_ids(chain.sectors())
    ))
}

pub(crate) fn format_sector_ids(sectors: &[u32]) -> String {
    if sectors.is_empty() {
        return "-".to_string();
    }

    sectors
        .iter()
        .map(|sector| sector.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_cfb_id(id: u32) -> String {
    if id == 0xffff_ffff {
        "-".to_string()
    } else {
        id.to_string()
    }
}
