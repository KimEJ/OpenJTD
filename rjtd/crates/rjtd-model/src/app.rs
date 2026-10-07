use crate::*;

/// Application-facing document facade with rendering and editing context.
///
/// rjtd does not yet have a full Ichitaro layout engine. This facade keeps the
/// same load/query/render direction while rendering the current document model
/// as plain text pages.
#[derive(Debug, Clone)]
pub struct DocumentCore {
    pub(crate) document: Document,
    pub(crate) pages: Vec<Vec<PageTextLine>>,
    pub(crate) file_name: String,
    pub(crate) print_date: Option<String>,
    pub(crate) dpi: f64,
    pub(crate) page_layout: PageLayout,
    pub(crate) show_paragraph_marks: bool,
    pub(crate) show_control_codes: bool,
    pub(crate) show_transparent_borders: bool,
    pub(crate) clip_enabled: bool,
    pub(crate) writing_mode: WritingMode,
    pub(crate) next_snapshot_id: u32,
    pub(crate) snapshots: Vec<DocumentSnapshot>,
    pub(crate) caret_section: u32,
    pub(crate) caret_paragraph: u32,
    pub(crate) caret_char_offset: u32,
    pub(crate) clipboard_text: Option<String>,
}

mod document_core_editing;

mod editor_navigation_support;
pub(crate) use editor_navigation_support::*;

mod footnote_field_editing;

mod object_shape_editing;

mod search_render_editing;

mod table_cell_editing;

mod print_context;

#[derive(Debug, Clone)]
pub(crate) struct DocumentSnapshot {
    pub(crate) id: u32,
    pub(crate) document: Document,
    pub(crate) pages: Vec<Vec<PageTextLine>>,
    pub(crate) file_name: String,
    pub(crate) dpi: f64,
    pub(crate) page_layout: PageLayout,
    pub(crate) show_paragraph_marks: bool,
    pub(crate) show_control_codes: bool,
    pub(crate) show_transparent_borders: bool,
    pub(crate) clip_enabled: bool,
    pub(crate) writing_mode: WritingMode,
    pub(crate) caret_section: u32,
    pub(crate) caret_paragraph: u32,
    pub(crate) caret_char_offset: u32,
    pub(crate) clipboard_text: Option<String>,
}

impl DocumentSnapshot {
    pub(crate) fn capture(id: u32, core: &DocumentCore) -> Self {
        Self {
            id,
            document: core.document.clone(),
            pages: core.pages.clone(),
            file_name: core.file_name.clone(),
            dpi: core.dpi,
            page_layout: core.page_layout,
            show_paragraph_marks: core.show_paragraph_marks,
            show_control_codes: core.show_control_codes,
            show_transparent_borders: core.show_transparent_borders,
            clip_enabled: core.clip_enabled,
            writing_mode: core.writing_mode,
            caret_section: core.caret_section,
            caret_paragraph: core.caret_paragraph,
            caret_char_offset: core.caret_char_offset,
            clipboard_text: core.clipboard_text.clone(),
        }
    }
}
