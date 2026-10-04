use rjtd_model::DocumentTextFlow;

use super::primitives::{push_json_string, push_u16_array_json};
use super::text_layout::push_text_source_span_json;

pub(crate) fn push_document_text_flow_json(output: &mut String, flow: &DocumentTextFlow) {
    output.push_str("{\"source\":");
    push_json_string(output, flow.source());
    output.push_str(",\"decoded\":false,\"sourceSpan\":");
    push_text_source_span_json(output, flow.source_span());
    output.push_str(",\"events\":[");
    for (index, event) in flow.events().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        push_json_string(output, event.kind().as_str());
        output.push_str(",\"sourceSpan\":");
        push_text_source_span_json(output, event.source_span());
        output.push_str(",\"text\":");
        push_json_string(output, event.text());
        for (name, value) in [
            ("code", event.code()),
            ("selector", event.selector()),
            ("recordClass", event.record_class()),
        ] {
            output.push_str(",\"");
            output.push_str(name);
            output.push_str("\":");
            match value {
                Some(value) => output.push_str(&value.to_string()),
                None => output.push_str("null"),
            }
        }
        output.push_str(",\"rawWords\":");
        push_u16_array_json(output, event.raw_words());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("]}");
}
