use crate::*;

pub(crate) struct NativeTocRow {
    fragments: [PageLayerTextFragment; 2],
    xs: [f32; 2],
    widths: [f32; 2],
    baseline: f32,
    font: f32,
    family: String,
    line: usize,
    leader: Option<(u16, TextSourceSpan)>,
}

/// Saved TOC rows in the corroborated title/leader/cache profile. Optional
/// leader records select body-right labels; absent records retain adjacent labels.
/// General tab stops, heading navigation, and printer leader metrics are unproven.
pub(crate) fn native_toc_row(
    document: &Document,
    layout: PageLayout,
    page: usize,
    mode: WritingMode,
    line: &PageTextLine,
    measured: &BTreeMap<usize, f32>,
) -> Option<NativeTocRow> {
    if document.toc_entries().is_empty()
        || mode.is_vertical()
        || !layout.has_source_margins()
        || modern_source_writing_mode(document) != Some(WritingMode::Horizontal)
        || !document.table_candidates().is_empty()
        || !document.object_frame_records().is_empty()
        || !document.footnote_text_candidates().is_empty()
        || !document.text_field_candidates().is_empty()
    {
        return None;
    }
    let record = line.native_line_mark_index?;
    let (owner, top, _) = native_rule_line_placement(document, layout, record)?;
    if owner != page {
        return None;
    }
    let resolver = document_text_style_resolver(document)?;
    let mut fragments: [PageLayerTextFragment; 2] =
        page_text_line_style_fragments(document, line, Some(&resolver))
            .try_into()
            .ok()?;
    for fragment in &mut fragments {
        fragment.source_span = Some(native_visible_text_span(
            document,
            &fragment.text,
            fragment.source_span.as_ref()?,
        )?);
    }
    let title = &fragments[0];
    let label = &fragments[1];
    document.toc_entries().iter().find(|entry| {
        entry.title() == title.text
            && entry.page_label() == label.text
            && entry.source_span().unit_start() == title.source_span.as_ref().unwrap().unit_start()
            && entry.source_span().unit_end() == label.source_span.as_ref().unwrap().unit_end()
    })?;
    if label.text.len() > 4
        || label.text.is_empty()
        || !label.text.bytes().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let records = document
        .document_text_flow()?
        .events()
        .iter()
        .filter(|event| {
            event.kind() == DocumentTextFlowKind::Record
                && title.source_span.as_ref().unwrap().unit_end() <= event.unit_start()
                && event.unit_end() <= label.source_span.as_ref().unwrap().unit_start()
        })
        .take(3)
        .collect::<Vec<_>>();
    if !matches!(records.len(), 1 | 2)
        || records[0].raw_words().get(2) != Some(&17)
        || records
            .iter()
            .any(|event| !native_toc_context_record(event))
    {
        return None;
    }
    let body = raw_stream_bytes(document, "/DocumentText")?;
    let mut next = title.source_span.as_ref()?.unit_end();
    for event in &records {
        if event.unit_start() != next {
            return None;
        }
        // The controlled title context reserves one fullwidth separator cell.
        // Require title cache 0005 and leader cache 0003; other objects stay fallback.
        let marker = if event.raw_words().len() == 17 { 5 } else { 3 };
        let cache = [0x1c, 1, 7, 0, 0, 1, 0x1d, marker, 0x1e, 5, 0, 1, 0x1f];
        if cache
            .into_iter()
            .enumerate()
            .any(|(i, word)| read_be16_at(body, (event.unit_end() + i) * 2) != Some(word))
        {
            return None;
        }
        next = event.unit_end().checked_add(cache.len())?;
    }
    if next != label.source_span.as_ref()?.unit_start() {
        return None;
    }
    let leader = if let Some(event) = records.get(1) {
        if event.raw_words().len() != 18 {
            return None;
        }
        Some((event.raw_words()[8], event.source_span().clone()))
    } else {
        None
    };
    let font = document_default_font_size_px(document)?;
    let mut family = None;
    let mut widths = [0.0; 2];
    for (index, fragment) in fragments.iter().enumerate() {
        let span = fragment.source_span.as_ref()?;
        let style = document_text_character_style(document, &resolver, span);
        if fragment.ruby_annotation.is_some()
            || style.bold
            || style.italic
            || style.underline
            || style.script.is_some()
            || (document_text_font_size(&resolver, span, Some(font))?.px - font).abs() > 0.01
            || document_text_foreground_color(&resolver, span).is_some_and(|c| c != "#000000")
        {
            return None;
        }
        let run_family = style
            .font
            .as_ref()
            .map_or_else(|| document_font_family_css(document), |(_, f)| f.clone());
        if family.as_ref().is_some_and(|f| f != &run_family) {
            return None;
        }
        family = Some(run_family);
        widths[index] = measured
            .get(&span.unit_start())
            .copied()
            .filter(|w| w.is_finite() && *w > 0.0)
            .unwrap_or_else(|| text_width_px_for_font_size(font, &fragment.text) as f32);
    }
    let title_x = layout.margin_left_px();
    let right = layout.width_px() - layout.margin_right_px();
    let title_end = title_x + widths[0] + font;
    let label_x = if leader.is_some() {
        right - widths[1]
    } else {
        title_end
    };
    if label_x < title_end || label_x + widths[1] > right {
        return None;
    }
    Some(NativeTocRow {
        fragments,
        xs: [title_x, label_x],
        widths,
        baseline: top + font,
        font,
        family: family?,
        line: record,
        leader,
    })
}

pub(crate) fn push_native_toc_svg(svg: &mut String, row: &NativeTocRow) {
    if let Some((kind, span)) = &row.leader {
        // Backend-neutral visual approximation, also used by the layer tree.
        // Source evidence establishes solid/dotted selection, not these metrics.
        let y = row.baseline - row.font * 0.35;
        let stroke = if *kind == 100 { 1.0 } else { 0.5 };
        let dash = if *kind == 100 {
            format!(
                " stroke-dasharray=\"0.1 {:.3}\" stroke-linecap=\"round\"",
                row.font / 3.0
            )
        } else {
            String::new()
        };
        svg.push_str(&format!("<line class=\"rjtd-native-toc-leader\" data-leader-kind-candidate=\"{kind}\" data-source-unit-start=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" x1=\"{:.3}\" y1=\"{y:.3}\" x2=\"{:.3}\" y2=\"{y:.3}\" stroke=\"#111111\" stroke-width=\"{stroke:.3}\"{dash}/>",span.unit_start(),row.xs[0]+row.widths[0]+row.font,row.xs[1]));
    }
    for (index, fragment) in row.fragments.iter().enumerate() {
        let unit = fragment.source_span.as_ref().unwrap().unit_start();
        svg.push_str(&format!("<text class=\"rjtd-native-toc-text\" id=\"rjtd-text-advance-{unit}\" data-source-unit-start=\"{unit}\" data-line-mark-record-index=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\" fill=\"#111111\" xml:space=\"preserve\">{}</text>",row.line,row.xs[index],row.baseline,escape_xml(&row.family),row.font,escape_xml(&fragment.text)));
    }
}

pub(crate) fn push_native_toc_layer_json(
    out: &mut String,
    sources: &mut Vec<String>,
    row: &NativeTocRow,
) {
    if let Some((kind, span)) = &row.leader {
        let stroke = if *kind == 100 { 1.0 } else { 0.5 };
        out.push_str(&format!(",{{\"type\":\"tocLeaderCandidate\",\"leaderKindCandidate\":{kind},\"x1\":{:.3},\"x2\":{:.3},\"y\":{:.3},\"strokeWidthCandidate\":{stroke:.3},\"dotPitchCandidate\":{:.3},\"sourceUnitStart\":{},\"sourceUnitEnd\":{},\"decoded\":false,\"geometryDecoded\":false}}",row.xs[0]+row.widths[0]+row.font,row.xs[1],row.baseline-row.font*0.35,row.font/3.0,span.unit_start(),span.unit_end()));
    }
    for (index, fragment) in row.fragments.iter().enumerate() {
        let id = sources.len();
        out.push_str(&format!(",{{\"type\":\"textRun\",\"bbox\":{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}},\"text\":{},\"baseline\":{:.3},\"fontSize\":{:.3},\"fontFamily\":{},\"lineMarkRecordIndex\":{},\"projectionKind\":\"nativeTocCandidate\",\"separatorCellsCandidate\":1,\"rightLabelCandidate\":{},\"decoded\":false,\"geometryDecoded\":false,\"positionsDecoded\":false,\"source\":",row.xs[index],row.baseline-row.font,row.widths[index],row.font,json_string(&fragment.text),row.baseline,row.font,json_string(&row.family),row.line,row.leader.is_some()));
        push_page_layer_source_span_json(out, id, fragment);
        out.push('}');
        push_page_layer_text_source_json(sources, id, fragment);
    }
}
