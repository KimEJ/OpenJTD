use rjtd_model::{
    ObjectImageDimensions, ObjectImageHeaderFieldCandidates, ObjectImageNumericHeaderField,
    ObjectImagePayloadEnvelope, ObjectImagePayloadSpan, ObjectImageSourcePathCandidate,
};

use super::primitives::{hex, push_json_string};

pub(crate) fn push_object_image_payload_span_json(
    output: &mut String,
    span: &ObjectImagePayloadSpan,
) {
    output.push_str("{\"kind\":");
    push_json_string(output, span.kind());
    output.push_str(",\"mime\":");
    push_json_string(output, span.mime());
    output.push_str(",\"signatureOffset\":");
    output.push_str(&span.signature_offset().to_string());
    output.push_str(",\"start\":");
    output.push_str(&span.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&span.end().to_string());
    output.push_str(",\"length\":");
    output.push_str(&span.len().to_string());
    output.push_str(",\"complete\":");
    output.push_str(if span.complete() { "true" } else { "false" });
    output.push_str(",\"dimensions\":");
    push_object_image_dimensions_json(output, span.dimensions());
    output.push_str(",\"objectEnvelope\":");
    push_object_image_payload_envelope_json(output, span.envelope());
    output.push_str(",\"payloadPrefixHex\":");
    push_json_string(
        output,
        &hex(&span.payload()[..span.payload().len().min(16)]),
    );
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_dimensions_json(
    output: &mut String,
    dimensions: Option<ObjectImageDimensions>,
) {
    if let Some(dimensions) = dimensions {
        output.push_str("{\"width\":");
        output.push_str(&dimensions.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&dimensions.height().to_string());
        output.push('}');
    } else {
        output.push_str("null");
    }
}

fn push_object_image_payload_envelope_json(
    output: &mut String,
    envelope: &ObjectImagePayloadEnvelope,
) {
    output.push_str("{\"headerStart\":");
    output.push_str(&envelope.header_start().to_string());
    output.push_str(",\"headerEnd\":");
    output.push_str(&envelope.header_end().to_string());
    output.push_str(",\"headerLength\":");
    output.push_str(&envelope.header_len().to_string());
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(
        output,
        &hex(&envelope.header()[..envelope.header().len().min(16)]),
    );
    output.push_str(",\"headerFields\":");
    push_object_image_header_fields_json(output, envelope.header_fields());
    output.push_str(",\"trailerStart\":");
    output.push_str(&envelope.trailer_start().to_string());
    output.push_str(",\"trailerEnd\":");
    output.push_str(&envelope.trailer_end().to_string());
    output.push_str(",\"trailerLength\":");
    output.push_str(&envelope.trailer_len().to_string());
    output.push_str(",\"trailerPrefixHex\":");
    push_json_string(
        output,
        &hex(&envelope.trailer()[..envelope.trailer().len().min(16)]),
    );
    output.push_str(",\"declaredPayloadLength\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.value().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthOffset\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.offset().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthEndian\":");
    if let Some(length) = envelope.declared_payload_length() {
        push_json_string(output, length.endian());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_header_fields_json(
    output: &mut String,
    fields: &ObjectImageHeaderFieldCandidates,
) {
    output.push_str("{\"u16LePrefix\":[");
    for (index, field) in fields.u16_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"u32LePrefix\":[");
    for (index, field) in fields.u32_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"sourcePathCandidate\":");
    if let Some(path) = fields.source_path_candidate() {
        push_object_image_source_path_candidate_json(output, path);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_numeric_header_field_json(
    output: &mut String,
    field: &ObjectImageNumericHeaderField,
) {
    output.push_str("{\"offset\":");
    output.push_str(&field.offset().to_string());
    output.push_str(",\"value\":");
    output.push_str(&field.value().to_string());
    output.push('}');
}

fn push_object_image_source_path_candidate_json(
    output: &mut String,
    path: &ObjectImageSourcePathCandidate,
) {
    output.push_str("{\"lengthOffset\":");
    output.push_str(&path.length_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&path.declared_length().to_string());
    output.push_str(",\"bytesStart\":");
    output.push_str(&path.bytes_start().to_string());
    output.push_str(",\"bytesEnd\":");
    output.push_str(&path.bytes_end().to_string());
    output.push_str(",\"nulTerminated\":");
    output.push_str(if path.nul_terminated() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bytesHex\":");
    push_json_string(output, &hex(path.bytes()));
    output.push_str(",\"textLossy\":");
    push_json_string(output, path.text_lossy());
    output.push_str(",\"decoded\":false}");
}
