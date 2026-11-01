//! `language-data/`: every file has its row in `ATTRIBUTION.md` (ADR 0004).

use std::path::Path;

/// Every file under `dir`, as a path relative to `root` with `/` separators, without dotfiles.
fn files(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let Ok(list) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in list.flatten() {
        let path = entry.path();
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if path.is_dir() {
            files(root, &path, out);
        } else if let Ok(rel) = path.strip_prefix(root) {
            let parts: Vec<String> = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            out.push(parts.join("/"));
        }
    }
}

#[test]
fn every_language_data_file_is_attributed() {
    let root = craft_i18n_tests::repo_root();
    let attribution = std::fs::read_to_string(root.join("ATTRIBUTION.md")).unwrap();
    let mut all = Vec::new();
    files(&root, &root.join("language-data"), &mut all);
    let missing: Vec<&String> = all
        .iter()
        .filter(|f| f.as_str() != "language-data/README.md")
        .filter(|f| !attribution.contains(&format!("| `{f}` |")))
        .collect();
    assert!(
        missing.is_empty(),
        "language-data files without an ATTRIBUTION.md row: {missing:?}"
    );
}

#[test]
fn every_language_data_row_has_a_licence_class() {
    let root = craft_i18n_tests::repo_root();
    let attribution = std::fs::read_to_string(root.join("ATTRIBUTION.md")).unwrap();
    for line in attribution
        .lines()
        .filter(|l| l.starts_with("| `language-data/"))
    {
        let class = line.split(" | ").nth(4).unwrap_or("");
        assert!(
            matches!(class, "Embeddable" | "Download-only"),
            "{line}: Licence class is {class:?}; Excluded or missing classes can't ship"
        );
        let path = line
            .trim_start_matches("| `")
            .split('`')
            .next()
            .unwrap_or("");
        assert!(root.join(path).is_file(), "{path}: row without a file");
    }
}
