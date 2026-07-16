# Third-Party Materials

The root [Apache License, Version 2.0](LICENSE) covers OpenJTD-authored source
code, build configuration, documentation, and purpose-built project assets. It
does not grant rights that OpenJTD's authors do not hold.

`third-party/` is intentionally excluded from Git and is used only for local
research reference material. Its contents are not part of the tracked OpenJTD
source release, are not designated as Apache-2.0 licensed, and remain governed
by their original licenses, notices, and other terms. Do not publish or add
such material to a distribution unless its terms permit doing so and all
required notices are retained.

The local Ichitaro OpenOffice filter reference is documented in
[RFC 0002](openjtd-spec/rfc/0002-ichitaro-openoffice-filter.md). Its included
Sun license controls that artifact; it is a clean-room compatibility reference,
not OpenJTD source code.

No `NOTICE` file is included solely for this boundary: no third-party material
is tracked in this source tree for redistribution. If permitted third-party
material is added later, preserve its required attribution and add a `NOTICE`
file only when that material's terms require one.

For the `rjtd-wasm` browser distribution workflow, locked compiled dependency
identities and archive-root source texts are recorded in
[`rjtd/licenses/wasm/DEPENDENCIES.lock.tsv`](rjtd/licenses/wasm/DEPENDENCIES.lock.tsv)
and [`rjtd/licenses/wasm/THIRD_PARTY_NOTICES.txt`](rjtd/licenses/wasm/THIRD_PARTY_NOTICES.txt).
The workflow copies the generated notice bundle to
`openjtd.github.io/pkg/THIRD_PARTY_NOTICES.txt` with the generated WebAssembly
package. This boundary records source identities and retained texts only; it
does not make a legal-sufficiency claim.
