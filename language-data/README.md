# Language data

The per-locale data the Crafting Apps check and break text with: spelling word lists and
hyphenation patterns ([ADR 0004](../docs/adr/0004-third-party-language-data.md),
[ADR 0005](../docs/adr/0005-language-data-lives-here.md); terms in [`GLOSSARY.md`](../GLOSSARY.md)).
The code that reads it is `craft-proof` in [craft-libs](https://github.com/storytold/craft-libs)
(`crates/proof`). Grammar rules and suggestion tuning are code, not Language data, and stay there.

## Layout

One directory per locale, `language-data/<locale>/`, named in lowercase BCP 47. A regional locale
(`en-gb`) falls back to its language (`en`) for anything it doesn't have.

| Path | What | Format |
|---|---|---|
| `en-us/en-us.dic` | Moby Hyphenator II word list (~165k words with break points): English spelling and hyphenation | WordCraft's compact word list: zlib-compressed, decoded by `craft-proof`'s `dict.rs` (not a Hunspell `.dic`) |
| `en-us/hyphenation.pat` | Our own US English hyphenation patterns, for words not in the list | Liang/TeX patterns |
| `en-us/en-extra.txt` | Common modern English words the Moby list lacks | one lowercase base form per line, `#` comments |
| `en-us/en-variants.txt` | American/British spelling pairs, with a policy per English | `POLICY AMERICAN BRITISH` per line, `#` comments |
| `de/hyphenation.pat`, `pl/hyphenation.pat`, `pt/hyphenation.pat`, `it/hyphenation.pat` | hyph-utf8 TeX hyphenation patterns (`hyph-de-1996.tex`, `hyph-pl.tex`, `hyph-pt.tex`, `hyph-it.tex`) | unmodified `.tex`, compiled when loaded |
| `ru/ru-extra.txt` | Common modern Russian words older Hunspell dictionaries lack, with their declension classes | one base form and class per line, `#` comments |

This is the data WordCraft embedded before the move, byte for byte (hyphenation patterns renamed
to `hyphenation.pat`, contents unchanged). It is the carry-over ADR 0004 and ADR 0005 allow: the
non-English files leave the default build in the release that ships Language packs.

**Interim.** `en-us.dic` (Moby), `en-extra.txt` and `en-variants.txt` stay under these names until
the SCOWL-based `spelling.aff`/`spelling.dic` for en-us and en-gb replace the Moby list, and the
Licence class lives in `ATTRIBUTION.md` until `manifest.tsv` lands. `hyphenation.pat` is the
final name.

## Rules

- **Unmodified upstream files.** Third-party files are kept exactly as published; our own
  corrections go in our own files (`*-extra.txt`), under this repository's licence.
- **Every file has a row in [`ATTRIBUTION.md`](../ATTRIBUTION.md)** (Language data section): author,
  source (with the upstream SHA-256 where there is one), licence and Licence class. The checker
  (`crates/craft-i18n-tests/tests/language_data.rs`) fails otherwise, and, until the licence table
  is signed off, on any class but Embeddable or a licence it gates (LGPL, MPL, LPPL, GPL, CC-BY-SA).
  Dotfiles count.
- **Bytes are kept.** `.gitattributes` marks `language-data/**` as `-text`, so no line-ending
  conversion touches the files.

## How the apps use it

Like the catalogs, it is an optional build input: `craft-proof`'s `build.rs` embeds the files it
names from `$CRAFT_I18N_DIR/language-data/`. Without `CRAFT_I18N_DIR` the crate still builds, with no
embedded Language data (it warns); `CRAFT_I18N_REQUIRED=1`, as releases set, makes that an error.
See [`docs/integration.md`](../docs/integration.md) for the variables.
