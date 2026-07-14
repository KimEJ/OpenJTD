# Security Policy

OpenJTD parses externally supplied JTD, JTT, and JTTC binary documents. Parser
crashes, hangs, excessive resource use, malformed output, and unsafe handling of
embedded content are in scope for security reports.

## Supported Versions

OpenJTD is pre-stable. Security fixes are made for the latest published `0.0.x`
release and the current `dev` branch. Older pre-stable releases are not
maintained after a fixed release is available.

| Version | Supported |
| --- | --- |
| Latest `0.0.x` release | Yes |
| Current `dev` branch | Best effort |
| Older releases | No |

## Reporting a Vulnerability

Use GitHub Private Vulnerability Reporting from the repository's
[Security advisories page](https://github.com/KimEJ/OpenJTD/security/advisories).
Do not disclose vulnerability details in a public issue, discussion, or pull
request.

If the private reporting button is unavailable, open a public issue that asks
the maintainers to establish a private contact. Do not include technical
details, proof-of-concept data, or affected documents in that issue.

Include, when available:

- the affected OpenJTD version or commit;
- the operating system and execution surface (library, CLI, or WASM);
- the affected document format and the command or API call used;
- minimal reproduction steps and observed resource usage;
- the expected impact and any known mitigation;
- a minimized synthetic reproducer, or instructions for arranging private
  transfer of a sensitive sample.

Do not upload confidential, personal, proprietary, or redistribution-restricted
documents to a public repository. Coordinate a private transfer before sharing
such a file.

## Handling and Disclosure

The maintainers will assess reports on a best-effort basis. OpenJTD does not
promise a fixed response SLA while it is pre-stable. Confirmed vulnerabilities
will be fixed in private when practical, followed by a patched release and a
GitHub security advisory when public disclosure helps downstream users.

Please allow time for triage and a fix before publishing details. Credit will be
offered to reporters who want it, subject to coordinated disclosure.

## Out of Scope

- layout differences that do not cross a security boundary;
- unsupported or undecoded format behavior already reported as diagnostic or
  fallback output;
- vulnerabilities in Ichitaro or other third-party products that do not affect
  OpenJTD;
- reports that require redistributing a document without the necessary rights.
