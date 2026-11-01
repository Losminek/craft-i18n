# Language data format

How the Language data other than English is described, checked and published as **Language
packs**: one downloadable archive per locale, with an index the apps pin. English (and the
interim carry-over listed in [`language-data/README.md`](../language-data/README.md)) is
committed and embedded; everything here is fetched from its upstream at a pinned commit and never
committed. Licence classes are [ADR 0004](adr/0004-third-party-language-data.md); why the data
lives here is [ADR 0005](adr/0005-language-data-lives-here.md). The audit the manifest was seeded
from is dated 2026-10-31.

## The manifest: `language-data/manifest.toml`

One `[[file]]` per upstream file:

```toml
[[file]]
locale = "nl"                 # lowercase BCP 47
kind = "spelling-aff"         # spelling-aff | spelling-dic | hyphenation | licence
url = "https://raw.githubusercontent.com/LibreOffice/dictionaries/<commit>/nl_NL/nl_NL.aff"
commit = "<upstream commit sha>"
sha256 = "<hex>"
size = 12345                  # bytes
licence = "BSD-3-Clause"      # SPDX expression
class = "Embeddable"          # Embeddable | Download-only | Excluded
name = "Nederlands"           # optional, first entry per locale wins: endonym
```

- **A pack is every entry of one locale.** `spelling-aff`/`spelling-dic` are a Hunspell pair,
  `hyphenation` a TeX pattern file (an unmodified `hyph-*.tex` from tex-hyphen, compiled when
  loaded), `licence` a licence or README text that travels with the data.
- **`licence`** is the licence the file is used under. When upstream offers several, the one in
  the least restricted class is recorded (ADR 0004); when the evidence is weak or contradictory,
  the whole statement is recorded and the class is Download-only. A `licence` entry carries the
  licence and class of the data it describes. Hyphenation files carry their licence in their own
  header, so they need no `licence` entry.
- **`class`** may be more restricted than the licence allows (weak evidence), never less.
- Sources are pinned raw files: LibreOffice/dictionaries, tex-hyphen, and an upstream repository
  or wooorm/dictionaries where the audit names it the best source.

### Licence allow-list

Every SPDX id in an expression must be in the checker's allow-list
(`crates/craft-i18n-tests/src/packs.rs`, `LICENCES`), which gives each its class. `OR` takes
the less restricted side and `AND` the more restricted. A licence with no SPDX id gets a
`LicenseRef-` id there first (today `LicenseRef-BSD-Lebedev`, the Russian dictionary's BSD
licence with a "modified versions must be marked" clause, and `LicenseRef-hyph-bg`, the
Bulgarian patterns' permissive licence). `WITH` exceptions and `+` are not allowed: write
`-or-later`.

## Check words: `language-data/checks/<locale>.toml`

```toml
correct = ["huis", "boek", "school", "water", "stad"]   # at least 5
incorrect = ["boekk", "schooll", "werkken"]             # at least 3
hidden = true                                          # optional
reason = "craft-proof can't read two-character flags yet (FLAG long)"
```

Common words, written here (this repository's licence). A regional locale without its own file
uses its language's (`es-mx` uses `es`). **`hidden = true`** keeps a pack out of releases while
`craft-proof` can't read it; it needs a `reason`. Each reason has an engine issue in craft-libs.

## Checks

Every CI run (`cargo test --workspace`, no network):

- the manifest parses; every entry has a valid lowercase BCP 47 locale (no English), a pinned
  `https://raw.githubusercontent.com/` URL containing its 40-digit `commit`, a 64-digit lowercase
  `sha256`, a size of 1 byte to 64 MiB (`craft-proof`'s limit), a licence and a class;
- every licence parses as SPDX, uses only allow-listed ids, and its class is not less restricted
  than the licence allows;
- no duplicate (locale, kind), and no two licence texts with the same file name in a locale;
- a locale with spelling has both `.aff` and `.dic` and a licence text; every locale has a name;
- every locale with spelling that isn't Excluded has check words (or is hidden with a reason),
  and every checks file names a manifest locale;
- no locale with an Excluded entry is published.

With `CRAFT_I18N_FETCH=1` (the weekly and release workflow,
[`.github/workflows/language-packs.yml`](../.github/workflows/language-packs.yml)):

- every file downloads and matches its `size` and `sha256` (`every_file_matches_its_pin`);
- every published pack's `.aff`/`.dic` loads through `craft-proof`'s affix engine, accepts every
  `correct` word and rejects every `incorrect` one, and its hyphenation patterns compile
  (`packs_load_and_check`). Hidden packs are tried too and reported, so the log says when one
  could be published.

Files are cached by sha256 in `$CRAFT_I18N_CACHE` (default `target/language-data-cache`).

## Release assets

A craft-i18n GitHub release `language-packs-<n>` (pushed tag, or the workflow run by hand with a
release name) holds:

- **`<locale>.tar.zst`** per published pack (no Excluded entry, not hidden, checks passed). At
  the archive root: `spelling.aff`, `spelling.dic`, `hyphenation.pat` (any may be absent),
  `LICENCES/<original file name>` and `pack.toml` (locale, name, licence, class, version, and a
  `[[source]]` per file with its url, commit, sha256, licence and class). Deterministic: sorted
  names, zero times and owners, mode 0644, zstd level 19.
- **`index.json`**:
  ```json
  { "format": 1, "release": "language-packs-1",
    "packs": [ { "locale": "nl", "name": "Nederlands", "version": "<craft-i18n commit>",
                 "url": "https://github.com/storytold/craft-i18n/releases/download/language-packs-1/nl.tar.zst",
                 "sha256": "<hex of archive>", "size": 123456,
                 "licence": "BSD-3-Clause", "class": "Embeddable",
                 "spelling": true, "hyphenation": true } ] }
  ```
  A pack's `licence` is its files' distinct licences joined with `AND`; its `class` its most
  restricted file's.

Apps pin the index by URL + sha256; packs have no update checks of their own.

## The `language-packs` tool

```sh
cargo run --release --bin language-packs -- check     # the offline checks
cargo run --release --bin language-packs -- fetch     # download + verify every file into the cache
cargo run --release --bin language-packs -- build --out DIR --release NAME --version SHA \
    [--base-url URL] [--only nl,fr,pl] [--cache DIR] [--no-download]
```

`--base-url` defaults to the release's download URL; `--only` builds just those locales (and
fails on a hidden or Excluded one).

### Testing an index locally

To try an app against packs before a release is published, build them with archive URLs on a
local server and serve the folder:

```sh
cargo run --release --bin language-packs -- build --out /tmp/packs \
    --release language-packs-test --version "$(git rev-parse HEAD)" \
    --base-url http://127.0.0.1:8765/ --only pl,ru,it
python3 -m http.server 8765 --bind 127.0.0.1 --directory /tmp/packs
```

Then point the app's pack index at `http://127.0.0.1:8765/index.json` with the sha256 of that
file (`sha256sum /tmp/packs/index.json`). Leave out `--only` to build every published pack.
