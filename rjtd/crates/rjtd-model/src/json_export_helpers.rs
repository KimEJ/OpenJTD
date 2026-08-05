use super::*;

pub(crate) fn json_ok_with(fields: &str) -> String {
    format!("{{\"ok\":true,{fields}}}")
}

pub(crate) fn default_cursor_rect_json(page_index: u32) -> String {
    format!(
        "{{\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1}}}",
        page_index, APP_PAGE_MARGIN_PX, APP_PAGE_MARGIN_PX, APP_LINE_HEIGHT_PX
    )
}

pub(crate) fn default_line_info_json() -> String {
    "{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":0}".to_string()
}

pub(crate) fn default_table_dimensions_json() -> String {
    "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}".to_string()
}

pub(crate) fn observed_table_dimensions_json(candidate: &TableCandidate) -> String {
    let row_count = candidate.intervals().len();
    let mut output = format!(
        "{{\"rowCount\":{row_count},\"colCount\":1,\"cellCount\":{row_count},\"source\":\"tableCandidate\",\"tableCandidateIndex\":{},\"basis\":\"{}\",\"delimiterCode\":{},\"delimiterCodeHex\":\"0x{:04x}\",\"columnSplitCandidateRows\":{},\"maxColumnSegmentCount\":{},\"columnSegmentPatternConsistent\":{},\"columnSegmentPatternMismatchRows\":{}",
        candidate.index(),
        candidate.basis().as_str(),
        candidate.delimiter_code(),
        candidate.delimiter_code(),
        candidate.column_split_candidate_row_count(),
        candidate.max_column_segment_count(),
        if candidate.column_segment_pattern_consistent() {
            "true"
        } else {
            "false"
        },
        candidate.column_segment_pattern_mismatch_rows()
    );
    output.push_str(",\"columnGridCandidate\":");
    if let Some(grid) = candidate.column_segment_grid_candidate() {
        output.push_str(&column_grid_candidate_json(candidate, &grid));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"columnSplittingDecoded\":false,\"decoded\":false}");
    output
}

pub(crate) fn column_grid_candidate_json(
    candidate: &TableCandidate,
    grid: &TableCandidateColumnGridCandidate,
) -> String {
    let pattern = grid
        .pattern()
        .iter()
        .map(|kind| json_string(kind.as_str()))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"source\":\"columnSegments\",\"tableCandidateIndex\":{},\"rowCount\":{},\"colCountCandidate\":{},\"cellCountCandidate\":{},\"columnSplitCandidateRows\":{},\"maxColumnSegmentCount\":{},\"columnSegmentPatternConsistent\":true,\"columnSegmentPatternMismatchRows\":0,\"pattern\":[{}],\"geometryDecoded\":false,\"decoded\":false}}",
        candidate.index(),
        grid.row_count(),
        grid.column_count(),
        grid.cell_count(),
        grid.split_row_count(),
        candidate.max_column_segment_count(),
        pattern
    )
}

pub(crate) fn default_cell_info_json() -> String {
    "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}".to_string()
}

pub(crate) fn observed_cell_info_json(cell_idx: u32, cell: &TableCandidateInterval) -> String {
    format!(
        "{{\"row\":{cell_idx},\"col\":0,\"rowSpan\":1,\"colSpan\":1,\"source\":\"tableCandidateInterval\",\"sourceIntervalIndex\":{},\"sourceStart\":{},\"sourceEnd\":{},\"decoded\":false}}",
        cell.source_interval_index(),
        cell.source_start(),
        cell.source_end()
    )
}

pub(crate) fn observed_cell_line_info_json(cell: &TableCandidateInterval) -> String {
    let char_end = cell.text_preview().chars().count();
    format!("{{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":{char_end}}}")
}

pub(crate) fn observed_table_signature(candidate: &TableCandidate) -> String {
    format!(
        "rjtd-table-candidate:{}:{}:0x{:04x}:{}x1",
        candidate.index(),
        candidate.basis().as_str(),
        candidate.delimiter_code(),
        candidate.intervals().len()
    )
}

pub(crate) fn char_slice(text: &str, char_offset: u32, count: u32) -> String {
    text.chars()
        .skip(char_offset as usize)
        .take(count as usize)
        .collect()
}

pub(crate) fn default_table_edit_result_json() -> String {
    "{\"ok\":false,\"rowCount\":0,\"colCount\":0}".to_string()
}

pub(crate) fn default_cell_count_result_json() -> String {
    "{\"ok\":false,\"cellCount\":0}".to_string()
}

pub(crate) fn default_char_properties_json() -> String {
    "{\"fontFamily\":\"Hiragino Sans\",\"fontName\":\"Hiragino Sans\",\"fontSize\":1000,\"bold\":false,\"italic\":false,\"underline\":false,\"strikethrough\":false,\"textColor\":\"#111111\",\"shadeColor\":\"#ffffff\",\"charShapeId\":0,\"fontId\":0,\"fontIds\":[0,0,0,0,0,0,0],\"fontFamilies\":[\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\"],\"ratios\":[100,100,100,100,100,100,100],\"spacings\":[0,0,0,0,0,0,0],\"relativeSizes\":[100,100,100,100,100,100,100],\"charOffsets\":[0,0,0,0,0,0,0],\"underlineType\":\"None\",\"underlineColor\":\"#111111\",\"outlineType\":0,\"shadowType\":0,\"shadowColor\":\"#000000\",\"shadowOffsetX\":0,\"shadowOffsetY\":0,\"strikeColor\":\"#111111\",\"subscript\":false,\"superscript\":false,\"emphasisDot\":0,\"underlineShape\":0,\"strikeShape\":0,\"kerning\":false,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0}".to_string()
}

pub(crate) fn default_para_properties_json() -> String {
    "{\"alignment\":\"left\",\"lineSpacing\":160,\"lineSpacingType\":\"Percent\",\"marginLeft\":0,\"marginRight\":0,\"indent\":0,\"spacingBefore\":0,\"spacingAfter\":0,\"paraShapeId\":0,\"headType\":\"None\",\"paraLevel\":0,\"numberingId\":0,\"widowOrphan\":false,\"keepWithNext\":false,\"keepLines\":false,\"pageBreakBefore\":false,\"fontLineHeight\":false,\"singleLine\":false,\"autoSpaceKrEn\":false,\"autoSpaceKrNum\":false,\"verticalAlign\":0,\"englishBreakUnit\":0,\"koreanBreakUnit\":0,\"tabAutoLeft\":true,\"tabAutoRight\":true,\"tabStops\":[],\"defaultTabSpacing\":0,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0,\"borderSpacing\":[0,0,0,0]}".to_string()
}

pub(crate) fn default_cell_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"paddingLeft\":0,\"paddingRight\":0,\"paddingTop\":0,\"paddingBottom\":0,\"verticalAlign\":0,\"textDirection\":0,\"isHeader\":false,\"cellProtect\":false,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0}".to_string()
}

pub(crate) fn default_table_properties_json() -> String {
    "{\"cellSpacing\":0,\"paddingLeft\":0,\"paddingRight\":0,\"paddingTop\":0,\"paddingBottom\":0,\"pageBreak\":0,\"repeatHeader\":false,\"tableWidth\":0,\"tableHeight\":0,\"outerLeft\":0,\"outerRight\":0,\"outerTop\":0,\"outerBottom\":0,\"hasCaption\":false,\"treatAsChar\":false,\"textWrap\":\"topAndBottom\",\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"restrictInPage\":false,\"allowOverlap\":false,\"keepWithAnchor\":false,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0}".to_string()
}

pub(crate) fn default_picture_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"treatAsChar\":false,\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"textWrap\":\"topAndBottom\",\"brightness\":0,\"contrast\":0,\"effect\":\"none\",\"description\":\"\",\"rotationAngle\":0,\"horzFlip\":false,\"vertFlip\":false,\"originalWidth\":0,\"originalHeight\":0,\"cropLeft\":0,\"cropTop\":0,\"cropRight\":0,\"cropBottom\":0,\"paddingLeft\":0,\"paddingTop\":0,\"paddingRight\":0,\"paddingBottom\":0,\"outerMarginLeft\":0,\"outerMarginTop\":0,\"outerMarginRight\":0,\"outerMarginBottom\":0,\"borderColor\":0,\"borderWidth\":0,\"hasCaption\":false,\"captionDirection\":\"bottom\",\"captionVertAlign\":\"top\",\"captionWidth\":0,\"captionSpacing\":0,\"captionMaxWidth\":0,\"captionIncludeMargin\":false}".to_string()
}

pub(crate) fn default_shape_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"treatAsChar\":false,\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"textWrap\":\"topAndBottom\",\"tbMarginLeft\":0,\"tbMarginRight\":0,\"tbMarginTop\":0,\"tbMarginBottom\":0,\"tbVerticalAlign\":\"top\",\"borderColor\":0,\"borderWidth\":0,\"borderAttr\":0,\"borderOutlineStyle\":0,\"lineType\":0,\"lineEndShape\":0,\"arrowStart\":0,\"arrowEnd\":0,\"arrowStartSize\":0,\"arrowEndSize\":0,\"rotationAngle\":0,\"horzFlip\":false,\"vertFlip\":false,\"fillType\":\"none\",\"fillBgColor\":16777215,\"fillPatColor\":0,\"fillPatType\":0,\"fillAlpha\":0,\"gradientType\":0,\"gradientAngle\":0,\"gradientCenterX\":0,\"gradientCenterY\":0,\"gradientBlur\":0,\"roundRate\":0,\"description\":\"\"}".to_string()
}

pub(crate) fn default_equation_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"treatAsChar\":true,\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"textWrap\":\"topAndBottom\",\"zOrder\":0,\"instanceId\":0,\"outerMarginLeft\":0,\"outerMarginTop\":0,\"outerMarginRight\":0,\"outerMarginBottom\":0,\"hasCaption\":false,\"captionDirection\":\"bottom\",\"captionWidth\":0,\"captionSpacing\":0,\"description\":\"\",\"script\":\"\",\"fontSize\":1000,\"color\":0,\"baseline\":0,\"fontName\":\"Hiragino Sans\"}".to_string()
}

pub(crate) fn default_endnote_shape_json() -> String {
    "{\"ok\":false,\"numberFormat\":\"digit\",\"userChar\":\"\",\"prefixChar\":\"\",\"suffixChar\":\"\",\"startNumber\":1,\"separatorEnabled\":false,\"separatorLength\":0,\"separatorMarginTop\":0,\"separatorMarginBottom\":0,\"noteSpacing\":0,\"separatorLineType\":0,\"separatorLineWidth\":0,\"separatorColor\":\"#000000\",\"numbering\":\"continue\",\"placement\":\"documentEnd\"}".to_string()
}

pub(crate) fn json_string(value: &str) -> String {
    let mut escaped = String::new();
    escaped.push('"');
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character < ' ' => {
                escaped.push_str("\\u");
                escaped.push_str(&format!("{:04x}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}

pub(crate) fn json_bool(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

pub(crate) fn push_option_u32_hex_or_null_json(output: &mut String, value: Option<u32>) {
    match value {
        Some(value) => output.push_str(&json_string(&format!("0x{value:08x}"))),
        None => output.push_str("null"),
    }
}

pub(crate) fn non_negative_i32_offset(
    field_name: &'static str,
    value: i32,
) -> Option<(&'static str, usize)> {
    (value >= 0).then_some((field_name, value as usize))
}

pub(crate) fn push_usize_array_json(output: &mut String, values: &[usize]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

pub(crate) fn push_optional_usize_array_json(output: &mut String, values: &[Option<usize>]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_option_usize_json(output, *value);
    }
    output.push(']');
}

pub(crate) fn uniform_usize_stride(values: &[usize]) -> Option<usize> {
    if values.len() < 2 {
        return None;
    }
    let stride = values[1].checked_sub(values[0])?;
    if stride == 0 {
        return None;
    }
    values
        .windows(2)
        .all(|pair| pair[1].checked_sub(pair[0]) == Some(stride))
        .then_some(stride)
}

pub(crate) fn push_u16_array_json(output: &mut String, values: &[u16]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

pub(crate) fn push_u16_hex_array_json(output: &mut String, values: &[u16]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{value:04x}")));
    }
    output.push(']');
}

pub(crate) fn push_i32_array_json(output: &mut String, values: &[i32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

pub(crate) fn push_bool_array_json(output: &mut String, values: &[bool]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(if *value { "true" } else { "false" });
    }
    output.push(']');
}

pub(crate) fn push_sparse_observed_table_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&candidate.cell_count_candidate().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"rows\":");
    push_sparse_table_rows_json(output, candidate.intervals());
    output.push_str(",\"topologyCandidate\":");
    if let Some(topology) = candidate.sparse_topology_candidate() {
        push_sparse_topology_candidate_json(output, candidate, &topology);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"geometryDecoded\":false,\"decoded\":false}");
}

pub(crate) fn push_sparse_topology_candidate_json(
    output: &mut String,
    candidate: &TableCandidate,
    topology: &TableCandidateSparseTopologyCandidate,
) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&topology.row_count().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&topology.max_column_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&topology.cell_count().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&topology.empty_cell_count().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&topology.non_empty_cell_count().to_string());
    output.push_str(",\"rows\":[");
    for (index, row) in topology.rows().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&row.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&row.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&row.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&row.source_end().to_string());
        output.push_str(",\"cellCount\":");
        output.push_str(&row.cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&row.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&row.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.first_non_empty_column_index());
        output.push_str(",\"lastNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.last_non_empty_column_index());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"columns\":[");
    for (index, column) in topology.columns().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&column.index().to_string());
        output.push_str(",\"observedCellCount\":");
        output.push_str(&column.observed_cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&column.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&column.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyRowIndex\":");
        push_option_usize_json(output, column.first_non_empty_row_index());
        output.push_str(",\"lastNonEmptyRowIndex\":");
        push_option_usize_json(output, column.last_non_empty_row_index());
        output.push_str(",\"sourceStart\":");
        push_option_usize_json(output, column.source_start());
        output.push_str(",\"sourceEnd\":");
        push_option_usize_json(output, column.source_end());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

pub(crate) fn hex_bytes(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

pub(crate) fn document_font_names(document: &Document) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = BTreeSet::new();

    for font in document.fonts() {
        let name = font.name().trim();
        if name.is_empty() || looks_like_font_descriptor(name) {
            continue;
        }
        if seen.insert(name.to_string()) {
            names.push(name.to_string());
        }
    }

    if names.is_empty() {
        names.push("Hiragino Sans".to_string());
    }
    names
}

pub(crate) fn primary_document_font_name(font_names: &[String]) -> &str {
    font_names
        .iter()
        .find(|name| looks_like_mincho_font(name))
        .or_else(|| {
            font_names
                .iter()
                .find(|name| looks_like_japanese_font(name))
        })
        .or_else(|| font_names.first())
        .map(String::as_str)
        .unwrap_or("Hiragino Sans")
}

pub(crate) fn document_font_family_css(document: &Document) -> String {
    let font_names = document_font_names(document);
    let primary = primary_document_font_name(&font_names).to_string();
    let mut ordered = Vec::new();
    push_font_family_with_aliases(&mut ordered, &primary);
    for name in &font_names {
        push_font_family_with_aliases(&mut ordered, name);
    }
    for fallback in [
        "Hiragino Mincho ProN",
        "YuMincho",
        "Yu Mincho",
        "Hiragino Sans",
        "Hiragino Kaku Gothic ProN",
        "Yu Gothic",
        "Meiryo",
        "Noto Sans CJK JP",
        "sans-serif",
    ] {
        ordered.push(fallback.to_string());
    }

    let mut seen = BTreeSet::new();
    ordered
        .into_iter()
        .filter(|name| seen.insert(name.clone()))
        .map(|name| css_font_family_name(&name))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn push_font_family_with_aliases(output: &mut Vec<String>, name: &str) {
    output.push(name.to_string());
    output.extend(font_family_aliases(name).into_iter().map(str::to_string));
}

pub(crate) fn font_family_aliases(name: &str) -> Vec<&'static str> {
    if name.contains("游明朝") {
        return vec!["YuMincho", "Yu Mincho", "Hiragino Mincho ProN"];
    }
    if name.contains("ＭＳ 明朝") || name.contains("MS Mincho") {
        return vec!["MS Mincho", "Hiragino Mincho ProN", "YuMincho", "Yu Mincho"];
    }
    if name.contains("明朝") || name.to_ascii_lowercase().contains("mincho") {
        return vec!["Hiragino Mincho ProN", "YuMincho", "Yu Mincho"];
    }
    if name.contains("ゴシック") || name.to_ascii_lowercase().contains("gothic") {
        return vec!["Yu Gothic", "Hiragino Sans", "Meiryo"];
    }
    Vec::new()
}

pub(crate) fn css_font_family_name(name: &str) -> String {
    if matches!(name, "serif" | "sans-serif" | "monospace") {
        return name.to_string();
    }
    format!("'{}'", name.replace('\\', "\\\\").replace('\'', "\\'"))
}

pub(crate) fn looks_like_mincho_font(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    name.contains("明朝") || name.contains('游') || lower.contains("mincho")
}

pub(crate) fn looks_like_japanese_font(name: &str) -> bool {
    name.chars().any(
        |character| matches!(character as u32, 0x3040..=0x30ff | 0x4e00..=0x9fff | 0xff00..=0xffef),
    )
}

pub(crate) fn looks_like_font_descriptor(name: &str) -> bool {
    matches!(name, "太字" | "斜体" | "太字 斜体")
}

pub(crate) fn string_array_json(values: &[String]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
    output
}

pub(crate) fn string_slice_array_json(values: &[&str]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
    output
}

pub(crate) fn font_table_json(fonts: &[DocumentFont]) -> String {
    let mut output = String::from("[");
    for (index, font) in fonts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sourceStream\":");
        output.push_str(&json_string(font.source_stream()));
        output.push_str(",\"id\":");
        output.push_str(&font.id().to_string());
        output.push_str(",\"offset\":");
        output.push_str(&font.offset().to_string());
        output.push_str(",\"name\":");
        output.push_str(&json_string(font.name()));
        output.push_str(",\"rawHex\":");
        output.push_str(&json_string(&hex_bytes(font.raw())));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
    output
}
pub(crate) fn auto_texts_json(auto_texts: &[DocumentAutoText]) -> String {
    let mut output = String::from("[");
    for (index, auto_text) in auto_texts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sourceStream\":");
        output.push_str(&json_string(auto_text.source_stream()));
        output.push_str(",\"offset\":");
        output.push_str(&auto_text.offset().to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(auto_text.text()));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
    output
}

pub(crate) fn toc_entries_json(entries: &[DocumentTocEntry]) -> String {
    let mut output = String::from("[");
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"title\":");
        output.push_str(&json_string(entry.title()));
        output.push_str(",\"pageLabel\":");
        output.push_str(&json_string(entry.page_label()));
        output.push_str(",\"sourceSpan\":");
        push_text_source_span_json(&mut output, entry.source_span());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
    output
}

pub(crate) fn writing_mode_decision_json(document: &Document, selected: WritingMode) -> String {
    let decoded_layout = page_layout_from_document(document);
    let source_layout_hint = source_document_layout_hint(document, decoded_layout);
    let document_view_candidate =
        writing_mode_candidate_from_document_view_styles(document.unknown_styles());
    let paper_mark_writing_mode_diagnostics =
        paper_mark_writing_mode_diagnostics(document.paper_marks());
    let paper_mark_candidate = paper_mark_writing_mode_diagnostics.candidate;
    let computed = source_layout_hint
        .as_ref()
        .map(|hint| hint.writing_mode)
        .unwrap_or(WritingMode::Horizontal);
    let decision_source = if selected != computed {
        "runtime-override"
    } else if source_layout_hint.is_some() {
        "source-document-layout-hint"
    } else {
        "default-horizontal"
    };
    let decision_source_backed = matches!(decision_source, "source-document-layout-hint");
    let document_view_first_code_hex = document_view_candidate
        .as_ref()
        .map(|candidate| json_string(&format!("0x{:04x}", candidate.first_record_code)))
        .unwrap_or_else(|| "null".to_string());
    let source_hint_basis = source_layout_hint
        .as_ref()
        .map(|hint| json_string(hint.basis))
        .unwrap_or_else(|| "null".to_string());
    let source_hint_override_decoded_layout = source_layout_hint
        .as_ref()
        .map(|hint| hint.override_decoded_layout)
        .unwrap_or(false);
    let source_hint_margin = source_layout_hint
        .and_then(|hint| hint.margin_override_px)
        .map(|margin| format!("{margin:.3}"))
        .unwrap_or_else(|| "null".to_string());
    let source_hint_wrap_columns = source_layout_hint
        .and_then(|hint| hint.vertical_wrap_columns_override)
        .map(|columns| columns.to_string())
        .unwrap_or_else(|| "null".to_string());
    let source_hint_mode = source_layout_hint.map(|hint| hint.writing_mode);
    let document_view_mode = document_view_candidate
        .as_ref()
        .map(|candidate| candidate.writing_mode);
    let document_view_disagrees = document_view_mode
        .map(|mode| mode != selected)
        .unwrap_or(false);
    let source_hint_disagrees = source_hint_mode
        .map(|mode| mode != selected)
        .unwrap_or(false);
    let paper_mark_disagrees = paper_mark_candidate
        .map(|mode| mode != selected)
        .unwrap_or(false);
    format!(
        "{{\"selected\":\"{}\",\"source\":{},\"decoded\":false,\"sourceBacked\":{},\"computedBeforeRuntimeOverride\":\"{}\",\"documentViewStylesCandidate\":{},\"documentViewStylesFirstRecordCodeHex\":{},\"sourceDocumentLayoutHintCandidate\":{},\"sourceDocumentLayoutHintBasis\":{},\"sourceDocumentLayoutHintOverridesDecodedLayout\":{},\"sourceDocumentLayoutHintMarginOverridePx\":{},\"sourceDocumentLayoutHintVerticalWrapColumnsOverride\":{},\"paperMarkCandidate\":{},\"paperMarkCandidateDecoded\":false,\"paperMarkFlagBit0VerticalCandidate\":{},\"paperMarkFlagBit17IndexStepCandidate\":{},\"paperMarkWritingModeCandidateEvidence\":{},\"paperMarkWritingModeCandidateBlockers\":{},\"documentViewStylesDisagreesWithSelected\":{},\"sourceDocumentLayoutHintDisagreesWithSelected\":{},\"paperMarkDisagreesWithSelected\":{}}}",
        selected.as_str(),
        json_string(decision_source),
        decision_source_backed,
        computed.as_str(),
        writing_mode_option_json(document_view_mode),
        document_view_first_code_hex,
        writing_mode_option_json(source_hint_mode),
        source_hint_basis,
        source_hint_override_decoded_layout,
        source_hint_margin,
        source_hint_wrap_columns,
        writing_mode_option_json(paper_mark_candidate),
        paper_mark_writing_mode_diagnostics.flag_bit0_vertical_candidate,
        paper_mark_writing_mode_diagnostics.flag_bit17_index_step_candidate,
        string_slice_array_json(&paper_mark_writing_mode_diagnostics.evidence),
        string_slice_array_json(&paper_mark_writing_mode_diagnostics.blockers),
        document_view_disagrees,
        source_hint_disagrees,
        paper_mark_disagrees
    )
}

pub(crate) fn writing_mode_option_json(mode: Option<WritingMode>) -> String {
    mode.map(|mode| json_string(mode.as_str()))
        .unwrap_or_else(|| "null".to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocumentViewWritingModeCandidate {
    pub(crate) writing_mode: WritingMode,
    pub(crate) first_record_code: u16,
}

// DocumentViewStyles record 0x1001 appears as the first sequential record in
// current vertical-writing Ginga samples, but also appears in horizontal
// reference-PDF samples such as tsaiten, tmogi3_2, success_data-test, and
// shanai_lan. Keep it diagnostic-only until the surrounding style semantics are
// decoded.
pub(crate) fn writing_mode_candidate_from_document_view_styles(
    styles: &[UnknownStyle],
) -> Option<DocumentViewWritingModeCandidate> {
    styles
        .iter()
        .filter(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
        .find_map(|style| {
            let first_record_code = summarize_style_stream(style.payload())
                .records()
                .first()?
                .code();
            let writing_mode = if first_record_code == 0x1001 {
                WritingMode::VerticalRl
            } else {
                WritingMode::Horizontal
            };
            Some(DocumentViewWritingModeCandidate {
                writing_mode,
                first_record_code,
            })
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PaperMarkWritingModeDiagnostics {
    pub(crate) candidate: Option<WritingMode>,
    pub(crate) flag_bit0_vertical_candidate: bool,
    pub(crate) flag_bit17_index_step_candidate: bool,
    pub(crate) evidence: Vec<&'static str>,
    pub(crate) blockers: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StyleCandidate {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) source_stream: String,
    pub(crate) source_record_index: usize,
    pub(crate) source_offset: usize,
    pub(crate) source_code: u16,
    pub(crate) payload_len: usize,
}

pub(crate) fn style_candidate_names_json(candidates: &[StyleCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&candidate.name));
    }
    output.push(']');
    output
}

pub(crate) fn push_style_candidate_json(output: &mut String, candidate: &StyleCandidate) {
    output.push_str("{\"id\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"name\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"englishName\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"type\":0,\"nextStyleId\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"paraShapeId\":0,\"charShapeId\":0,\"decoded\":false,\"jtdCandidate\":true");
    push_style_candidate_source_json(output, candidate);
    output.push('}');
}

pub(crate) fn style_candidate_detail_json(candidate: &StyleCandidate) -> String {
    let mut output = String::new();
    output.push_str("{\"id\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"name\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"englishName\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"type\":0,\"nextStyleId\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"paraShapeId\":0,\"charShapeId\":0,\"decoded\":false,\"jtdCandidate\":true");
    push_style_candidate_source_json(&mut output, candidate);
    output.push_str(",\"charProps\":");
    output.push_str(&default_char_properties_json());
    output.push_str(",\"paraProps\":");
    output.push_str(&default_para_properties_json());
    output.push('}');
    output
}

pub(crate) fn style_at_candidate_json(candidate: &StyleCandidate) -> String {
    let mut output = String::new();
    output.push_str("{\"id\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"name\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"decoded\":false,\"jtdCandidate\":true");
    push_style_candidate_source_json(&mut output, candidate);
    output.push('}');
    output
}

pub(crate) fn push_style_candidate_source_json(output: &mut String, candidate: &StyleCandidate) {
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(&candidate.source_stream));
    output.push_str(",\"sourceRecordIndex\":");
    output.push_str(&candidate.source_record_index.to_string());
    output.push_str(",\"sourceOffset\":");
    output.push_str(&candidate.source_offset.to_string());
    output.push_str(",\"sourceCode\":");
    output.push_str(&candidate.source_code.to_string());
    output.push_str(",\"sourceCodeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", candidate.source_code)));
    output.push_str(",\"payloadLength\":");
    output.push_str(&candidate.payload_len.to_string());
}

pub(crate) fn style_source_streams_json(styles: &[UnknownStyle]) -> String {
    let mut output = String::from("[");

    for (index, style) in styles.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let summary = summarize_style_stream(style.payload());
        output.push_str("{\"name\":");
        match style.name() {
            Some(name) => output.push_str(&json_string(name)),
            None => output.push_str("null"),
        }
        output.push_str(",\"size\":");
        output.push_str(&style.payload().len().to_string());
        output.push_str(",\"family\":");
        output.push_str(&json_string(summary.family().as_str()));
        output.push_str(",\"headerU32Be\":");
        push_u32_array_json(&mut output, summary.header_u32_be());
        output.push_str(",\"headerU16Be\":");
        push_u16_array_json(&mut output, summary.header_u16_be());
        output.push_str(",\"recordLayout\":");
        output.push_str(&json_string(summary.record_layout().as_str()));
        output.push_str(",\"recordCount\":");
        output.push_str(&summary.records().len().to_string());
        output.push_str(",\"records\":");
        push_style_records_json(&mut output, summary.records());
        output.push_str(",\"decoded\":false}");
    }

    output.push(']');
    output
}

pub(crate) fn push_u32_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

pub(crate) fn push_u32_hex_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{value:02x}")));
    }
    output.push(']');
}

pub(crate) fn push_u32_hex8_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{value:08x}")));
    }
    output.push(']');
}

pub(crate) fn push_option_usize_json(output: &mut String, value: Option<usize>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

pub(crate) fn push_optional_usize_range_json(
    output: &mut String,
    start: Option<usize>,
    end: Option<usize>,
) {
    match (start, end) {
        (Some(start), Some(end)) => {
            output.push_str("{\"start\":");
            output.push_str(&start.to_string());
            output.push_str(",\"end\":");
            output.push_str(&end.to_string());
            output.push('}');
        }
        _ => output.push_str("null"),
    }
}

pub(crate) fn push_option_u16_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

pub(crate) fn push_option_u16_hex_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&json_string(&format!("0x{value:04x}"))),
        None => output.push_str("null"),
    }
}

pub(crate) fn push_optional_f32_json(output: &mut String, value: Option<f32>) {
    match value {
        Some(value) if value.is_finite() => output.push_str(&format!("{value:.3}")),
        _ => output.push_str("null"),
    }
}

pub(crate) fn push_optional_bbox_milli_json(
    output: &mut String,
    x_min_milli: Option<i32>,
    y_min_milli: Option<i32>,
    x_max_milli: Option<i32>,
    y_max_milli: Option<i32>,
) {
    let (Some(x_min_milli), Some(y_min_milli), Some(x_max_milli), Some(y_max_milli)) =
        (x_min_milli, y_min_milli, x_max_milli, y_max_milli)
    else {
        output.push_str("null");
        return;
    };
    let x = x_min_milli as f32 / 1000.0;
    let y = y_min_milli as f32 / 1000.0;
    let width = (x_max_milli - x_min_milli).max(0) as f32 / 1000.0;
    let height = (y_max_milli - y_min_milli).max(0) as f32 / 1000.0;
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
}

pub(crate) fn push_option_u32_json(output: &mut String, value: Option<u32>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

pub(crate) fn push_style_records_json(output: &mut String, records: &[StyleStreamRecordSummary]) {
    output.push('[');
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"offset\":");
        output.push_str(&record.offset().to_string());
        output.push_str(",\"code\":");
        output.push_str(&record.code().to_string());
        output.push_str(",\"codeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", record.code())));
        output.push_str(",\"payloadLength\":");
        output.push_str(&record.payload_len().to_string());
        output.push_str(",\"label\":");
        match record.label() {
            Some(label) => output.push_str(&json_string(label)),
            None => output.push_str("null"),
        }
        output.push_str(",\"subrecordCount\":");
        output.push_str(&record.subrecords().len().to_string());
        output.push_str(",\"subrecords\":");
        push_style_subrecords_json(output, record.subrecords());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_style_subrecords_json(
    output: &mut String,
    records: &[StyleStreamSubrecordSummary],
) {
    output.push('[');
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"offset\":");
        output.push_str(&record.offset().to_string());
        output.push_str(",\"code\":");
        output.push_str(&record.code().to_string());
        output.push_str(",\"codeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", record.code())));
        output.push_str(",\"payloadLength\":");
        output.push_str(&record.payload_len().to_string());
        output.push_str(",\"payloadHex\":");
        output.push_str(&json_string(&hex_bytes(record.payload())));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}
