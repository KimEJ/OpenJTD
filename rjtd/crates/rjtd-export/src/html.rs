use rjtd_model::{Block, Document, Inline};

pub fn to_html(document: &Document) -> String {
    let mut output = String::new();
    output.push_str(
        "<!DOCTYPE html>\n<html lang=\"ja\">\n<head><meta charset=\"UTF-8\"></head>\n<body>\n",
    );

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                output.push_str("<p>");
                for inline in paragraph.inlines() {
                    push_inline_html(&mut output, inline);
                }
                output.push_str("</p>\n");
            }
            Block::Unknown(_) => {}
        }
    }

    output.push_str("</body>\n</html>\n");
    output
}

fn push_inline_html(output: &mut String, inline: &Inline) {
    match inline {
        Inline::Text(text) => push_html_escaped(output, text.text()),
        Inline::Ruby(ruby) => {
            output.push_str("<ruby>");
            push_html_escaped(output, ruby.base_text());
            output.push_str("<rt>");
            push_html_escaped(output, ruby.annotation_text());
            output.push_str("</rt></ruby>");
        }
        Inline::Unknown(_) => {}
    }
}

fn push_html_escaped(output: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            _ => output.push(ch),
        }
    }
}
