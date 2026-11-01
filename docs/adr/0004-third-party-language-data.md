---
status: proposed
---

# Third-party Language data is allowed; translations stay clean room

Spelling dictionaries and hyphenation patterns can't be written from the plain meaning of the
English: they are decades of upstream work (Hunspell dictionaries, TeX hyphenation patterns). They
are taken from open-licensed upstream projects, unchanged, pinned and recorded in
`ATTRIBUTION.md`. The clean-room rule keeps applying to translations of the interface, which are
never copied. The suite word list is written here and is under the repository's own licence.

Every upstream file has a **Licence class**, recorded in the manifest next to its licence. The
class decides where the file may go:

| Licence class | Licences | Embedded in a binary | In a Language pack |
|---|---|---|---|
| **Embeddable** | Public domain, CC0, MIT, BSD, Apache-2.0, Unicode-3.0, SCOWL-style permissive, CC-BY 4.0; MPL and LPPL when unmodified | yes | yes |
| **Download-only** | LGPL, GPL, AGPL, CC-BY-SA | no | yes, as a separate unmodified file |
| **Excluded** | no licence, non-commercial, no-derivatives, or contradictory statements the author hasn't resolved | no | no |

- Files are never modified. LPPL patterns are shipped as the unmodified `.tex` and compiled when
  loaded, so no "Compiled Work" is distributed. Our own corrections go in our own files (for
  example `en-extra.txt`), under this repository's licence.
- When a file is offered under several licences, the most permissive one is used and recorded.
- A file whose licence evidence is weak or contradictory is **Download-only** until its author
  confirms otherwise (and **Excluded** when one of the statements is non-commercial or
  no-derivatives).
- The licence name and source link show in the app's language list before install, and the full
  licence text of everything embedded or installed shows in About › Language data. No separate
  licence prompt.
- CI rejects a file with no recorded licence or class, any Excluded file, and a Language build
  naming a pack that isn't Embeddable.

A maintainer signs this table off before the first Download-only language ships; it is not legal
advice. The audit behind it (2026-10-31, 66 spelling and 88 hyphenation rows, each with the
licence quoted) is the starting manifest.

## Considered options

- **Write our own dictionaries**: rejected, impossible at the quality users expect.
- **Permissive licences only**: rejected, it leaves out most Hunspell dictionaries (many are
  LGPL or MPL) and the languages users ask for most.
- **Embed LGPL with a notice**: rejected, LGPL lets users replace the file, which is awkward when
  it is compiled into a native or wasm binary and trivial when it is a separate file.
