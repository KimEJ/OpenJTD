# Architecture

OpenJTD は `rjtd` Rust ツール群を通じて JTD 固有のエンジンを構築する。原本解析、model の
所有、出力を階層に分離しても、他の形式の document model を採用する義務はない。
adapter 方針は [rhwp の参照・連携範囲](RHWP-COMPATIBILITY.ja.md) を参照する。

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

- Container code は logical streams を発見して開く。
- Stream code は byte-level の stream access を扱う。
- Record code は typed record と unknown record boundaries を decode する。
- Model code は semantic document structures を所有する。
- model の意味は JTD の原本根拠で決める。利用者固有の編集 address は原本解釈ではなく
  adapter 境界で扱う。
- Export code は document model だけを consume する。

Exporter は raw container、stream、record data を直接読んではならない。

## Unknown Preservation

リバースエンジニアリングは段階的に進む。まだ理解されていない data は破棄せず、unknown model shape のいずれかとして次の層へ運ぶ。

- `UnknownRecord`
- `UnknownBlock`
- `UnknownStyle`
- `UnknownObject`

## Current Implementation Boundary

現在の core block model は `Paragraph` と `Unknown`、inline は text、ruby、unknown
object を公開する。表、スタイル、レイアウト、オブジェクトの候補は、意味が証明されるまで
根拠として保持する。SVG/PDF 描画はこのモデルに fallback layout と限定的な診断投影を
組み合わせる。

`DocumentCore` は read/render API と基本的な本文編集を提供する。WASM の `HwpDocument`
wrapper は rhwp 形状の API に従うが、多くの高度な呼出しは既定値や no-op を返す。
API の存在は JTD 編集対応や round-trip preservation の証明にはならない。

基本的な文書 HTML 出力は `rjtd-export` に属し、app-core の HTML clipboard methods は
別の互換 surface である。現在の範囲と次の完了条件は [roadmap](ROADMAP.ja.md) を参照する。
