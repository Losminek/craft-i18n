//! `language-data/`: every file has exactly one row in `ATTRIBUTION.md`, with a licence and an
//! Embeddable Licence class (ADR 0004). The table stands in for the manifest until it lands.

use std::path::Path;

/// Licences that need the licence-table sign-off before they ship (ADR 0004). `GPL` also
/// matches LGPL and AGPL. ponytail: substring deny-list on a prose cell; SPDX allow-lists
/// replace it with the manifest.
const GATED: [&str; 4] = ["GPL", "MPL", "LPPL", "CC-BY-SA"];

/// Every file under `dir`, dotfiles included, as a path relative to `root` with `/` separators.
fn files(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let list = std::fs::read_dir(dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display()));
    for entry in list {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files(root, &path, out);
        } else {
            let rel = path.strip_prefix(root).unwrap();
            let parts: Vec<String> = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            out.push(parts.join("/"));
        }
    }
}

/// The problems with `files` (paths under `language-data/`) against `attribution`'s rows.
fn check(files: &[String], attribution: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let rows: Vec<(&str, &str, &str)> = attribution
        .lines()
        .filter(|l| l.starts_with("| `language-data/"))
        .map(|line| {
            let cells: Vec<&str> = line.split(" | ").collect();
            let path = line
                .trim_start_matches("| `")
                .split('`')
                .next()
                .unwrap_or("");
            (
                path,
                cells.get(3).map_or("", |c| c.trim()),
                cells.get(4).map_or("", |c| c.trim()),
            )
        })
        .collect();
    for (path, licence, class) in &rows {
        if !files.iter().any(|f| f == path) {
            errors.push(format!("{path}: row without a file"));
        }
        if licence.is_empty() {
            errors.push(format!("{path}: no licence"));
        }
        // Only the licence used, not the alternatives the notes in brackets mention.
        let used = licence.split(" (").next().unwrap_or("");
        if let Some(g) = GATED.iter().find(|g| used.contains(*g)) {
            errors.push(format!(
                "{path}: {g} needs the ADR 0004 licence-table sign-off"
            ));
        }
        if *class != "Embeddable" {
            errors.push(format!(
                "{path}: Licence class is {class:?}; only Embeddable ships until Language packs do"
            ));
        }
    }
    for f in files.iter().filter(|f| *f != "language-data/README.md") {
        match rows.iter().filter(|r| r.0 == f).count() {
            0 => errors.push(format!("{f}: no ATTRIBUTION.md row")),
            1 => {}
            n => errors.push(format!("{f}: {n} ATTRIBUTION.md rows")),
        }
    }
    errors
}

#[test]
fn language_data_is_attributed_and_embeddable() {
    let root = craft_i18n_tests::repo_root();
    let attribution = std::fs::read_to_string(root.join("ATTRIBUTION.md")).unwrap();
    let mut all = Vec::new();
    files(&root, &root.join("language-data"), &mut all);
    assert!(all.len() > 1, "language-data/ is empty");
    let errors = check(&all, &attribution);
    assert!(errors.is_empty(), "language-data:\n{}", errors.join("\n"));
}

#[cfg(test)]
mod fixtures {
    use super::check;

    const HEAD: &str =
        "| File | Author | Source | Licence | Licence class | Notes |\n|---|---|---|---|---|---|\n";

    fn run(files: &[&str], rows: &[&str]) -> Vec<String> {
        let files: Vec<String> = files.iter().map(|f| (*f).to_owned()).collect();
        check(&files, &format!("{HEAD}{}\n", rows.join("\n")))
    }

    fn row(path: &str, licence: &str, class: &str) -> String {
        format!("| `{path}` | someone | somewhere | {licence} | {class} | notes |")
    }

    #[test]
    fn clean() {
        let r = row(
            "language-data/de/hyphenation.pat",
            "MIT (offered under LPPL or MIT)",
            "Embeddable",
        );
        assert!(
            run(
                &[
                    "language-data/README.md",
                    "language-data/de/hyphenation.pat"
                ],
                &[&r]
            )
            .is_empty()
        );
    }

    #[test]
    fn each_failure() {
        let cases = [
            (
                vec!["language-data/de/.LICENSE"],
                vec![],
                "no ATTRIBUTION.md row",
            ),
            (
                vec![],
                vec![row("language-data/fr/x.dic", "MIT", "Embeddable")],
                "row without a file",
            ),
            (
                vec!["language-data/fr/x.dic"],
                vec![row("language-data/fr/x.dic", "", "Embeddable")],
                "no licence",
            ),
            (
                vec!["language-data/fr/x.dic"],
                vec![row("language-data/fr/x.dic", "LGPL-2.1-only", "Embeddable")],
                "GPL needs",
            ),
            (
                vec!["language-data/fr/x.dic"],
                vec![row("language-data/fr/x.dic", "MPL-2.0", "Embeddable")],
                "MPL needs",
            ),
            (
                vec!["language-data/fr/x.dic"],
                vec![row("language-data/fr/x.dic", "MIT", "Download-only")],
                "Licence class is",
            ),
            (
                vec!["language-data/fr/x.dic"],
                vec![row("language-data/fr/x.dic", "MIT", "Excluded")],
                "Licence class is",
            ),
            (
                vec!["language-data/fr/x.dic"],
                vec![
                    row("language-data/fr/x.dic", "MIT", "Embeddable"),
                    row("language-data/fr/x.dic", "MIT", "Embeddable"),
                ],
                "2 ATTRIBUTION.md rows",
            ),
        ];
        for (files, rows, want) in cases {
            let rows: Vec<&str> = rows.iter().map(String::as_str).collect();
            let errors = run(&files, &rows);
            assert!(
                errors.iter().any(|e| e.contains(want)),
                "{want}: got {errors:?}"
            );
        }
    }
}
