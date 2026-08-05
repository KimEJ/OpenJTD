use rjtd_model::{Block, Document, Inline};

pub fn to_plain_text(document: &Document) -> String {
    let mut output = String::new();

    for block in document.blocks() {
        if let Block::Paragraph(paragraph) = block {
            for inline in paragraph.inlines() {
                push_inline_visible_text(&mut output, inline);
            }
            output.push('\n');
        }
    }

    output
}

pub fn to_markdown(document: &Document) -> String {
    let mut output = String::new();

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                for inline in paragraph.inlines() {
                    push_inline_visible_text(&mut output, inline);
                }
                output.push_str("\n\n");
            }
            Block::Unknown(_) => {
                output.push_str("<!-- UnknownBlock preserved by rjtd -->\n\n");
            }
        }
    }

    output
}

fn push_inline_visible_text(output: &mut String, inline: &Inline) {
    match inline {
        Inline::Text(text) => output.push_str(text.text()),
        Inline::Ruby(ruby) => output.push_str(ruby.base_text()),
        Inline::Unknown(_) => {}
    }
}
