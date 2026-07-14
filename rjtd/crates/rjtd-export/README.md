# rjtd-export

Experimental text, Markdown, HTML, JSON, and PDF exporters for OpenJTD
documents.

`rjtd-export` is the export layer of
[OpenJTD](https://github.com/KimEJ/OpenJTD). It consumes `rjtd-model::Document`;
it does not parse source files directly.

## Developer preview

Version 0.0.1 is an experimental developer preview. All public APIs and output
details may change in any later 0.0.x release.

## Exports

- Plain text through `to_plain_text`.
- Markdown through `to_markdown`.
- Minimal HTML through `to_html`.
- Evidence-preserving JSON through `to_json`.
- Native PDF through `to_pdf` and `to_pdf_with_file_name` on non-WASM targets.

## Public API and PDF errors

`to_plain_text`, `to_markdown`, `to_html`, and `to_json` each take
`&rjtd_model::Document` and return a `String`. `to_pdf` and
`to_pdf_with_file_name` are available only on non-WASM targets and return
`Result<Vec<u8>, String>`.

The PDF `String` error is rendering diagnostics for the current conversion;
it is not a typed error value or a stable error-category contract. Treat its
text as display or logging context, not as a value to parse. On success, write
the returned bytes as a PDF; on error, no PDF bytes are returned.

## Example

```rust,no_run
let bytes = std::fs::read("document.jtd")?;
let document = rjtd_model::parse_document(&bytes)?;
let markdown = rjtd_export::to_markdown(&document);
println!("{markdown}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Output limits

Text-oriented output uses the currently decoded document model. JSON retains
`Candidate`, `Unknown`, and `Diagnostic` evidence, including fields marked
`decoded: false`; those values describe observed source data rather than final
document semantics. PDF and HTML use conservative fallbacks for layout that
has not been decoded. The exporters do not promise pixel fidelity, complete
feature coverage, or lossless round trips for 0.0.1.

## License

Apache-2.0.
