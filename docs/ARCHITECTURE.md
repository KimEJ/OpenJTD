# Architecture

OpenJTD currently builds its JTD engine through the `rjtd` Rust toolset. `rjtd`
follows the rhwp-style layered document engine architecture.

```text
Document File
      │
      ▼
Container Layer
      │
      ▼
Stream Layer
      │
      ▼
Record Layer
      │
      ▼
Document Model
      │
      ├──── Plain Text / Markdown Export
      ├──── HTML Export
      ├──── JSON Export
      └──── App Core / SVG / PDF Export
```

## Layer Rules

- Container code discovers and opens logical streams.
- Stream code handles byte-level stream access.
- Record code decodes typed and unknown record boundaries.
- Model code owns semantic document structures.
- Export code consumes only the document model.

Exporters must not read raw container, stream, or record data directly.

## Unknown Preservation

Reverse engineering is incremental. Any data that is not understood yet must be carried forward as one of the unknown model shapes instead of being discarded.

- `UnknownRecord`
- `UnknownBlock`
- `UnknownStyle`
- `UnknownObject`

## Current Implementation Boundary

The core block model currently exposes `Paragraph` and `Unknown`; inlines expose
text, ruby, and unknown objects. Preserved table, style, layout, and object
candidates carry evidence until their semantics are proven. SVG/PDF rendering
uses this model with fallback layout and limited diagnostic projections.

`DocumentCore` provides read/render APIs and basic body-text editing. The
`HwpDocument` WASM wrapper follows the rhwp-shaped API, but many advanced calls
return defaults or no-op results. API presence does not establish JTD editing
support or round-trip preservation.

Basic document HTML export belongs to `rjtd-export`; app-core HTML clipboard
methods are a separate compatibility surface. For current scope and the next
completion criteria, see the [roadmap](ROADMAP.md).
