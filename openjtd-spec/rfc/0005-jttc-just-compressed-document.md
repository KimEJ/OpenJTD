# RFC 0005: JTTC JustCompressedDocument Container

Status: draft for publication preparation; joint review pending

Japanese translation: [0005-jttc-just-compressed-document.ja.md](0005-jttc-just-compressed-document.ja.md)

## Observed Container Profile

Inspected JTTC files have an outer CFB with `/JSCompDocument`, rather than a
direct outer `/DocumentText`. The observed chain is:

```text
outer CFB -> /JSCompDocument -> JustCompressedDocument
          -> one LHA -lh5- member -> inner CFB -> /DocumentText
```

The wrapper prefix is:

```text
2600 4a75 7374 436f 6d70 7265 7373 6564 446f 6375 6d65 6e74
```

In the inspected profile the LHA member begins at stream byte 38.
Its decompressed bytes begin with CFB signature `d0 cf 11 e0 a1 b1 1a e1`.
Recorded examples have packed/original lengths 989292/1598976 and
1182377/1913856 bytes. These are observations, not fixed format constants.

## Inner Document

The inner CFB contains the logical document streams. `/DocumentText`, styles,
fonts, layout marks, fields, and objects must be investigated in that scope;
absence from the outer directory is not absence from the document.
Preserve the distinction between wrapper byte positions, decompressed CFB
positions, and logical-stream positions. A blank or control-heavy body does
not prove that auxiliary streams are absent.

## Unresolved Coverage and Validation

The recorded profile does not establish other LHA methods, multiple members,
all headers/checksums/CRC behavior, or every JTTC version. Compare a native
JTT/JTTC save pair by inner inventories, exact stream content, and independent
decompression. Record authoring version, hashes, rights, failures, and limits.
Decoder algorithms, allocation budgets, recovery policy, and repeated-reader
behavior belong in implementation records, not this format profile.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0005-jttc-just-compressed-document.md).
