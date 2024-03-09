use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snippet {
    pub id: usize,
    pub name: String,
    pub keyword: String,
    pub languague: String,
    pub snippet: String,
}

impl Snippet {
    pub fn new(
        id: usize,
        name: impl Into<String>,
        keyword: impl Into<String>,
        language: impl Into<String>,
        snippet: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            keyword: keyword.into(),
            languague: language.into(),
            snippet: snippet.into(),
        }
    }
}

pub fn get_snippets_dir() -> PathBuf {
    let mut path = PathBuf::new();

    if cfg!(target_os = "windows") {
        path.push(dirs::data_local_dir().unwrap());
    } else {
        path.push(dirs::config_dir().unwrap())
    }

    path.push("lighttigerxiv-wl-snippets");
    path
}

pub fn get_snippets_path() -> PathBuf {
    let mut path = get_snippets_dir();
    path.push("snippets.json");
    path
}

pub fn init_snippets() {
    if !get_snippets_dir().exists() {
        fs::create_dir_all(get_snippets_dir()).unwrap();
    }

    if !get_snippets_path().exists() {
        write_snippets(vec![]);
    }
}

pub fn write_snippets(snippets: Vec<Snippet>) {
    let json = serde_json::to_string_pretty(&snippets).unwrap();
    fs::write(get_snippets_path(), &json).unwrap();
}

pub fn get_snippets() -> Vec<Snippet> {
    let json = fs::read_to_string(get_snippets_path()).unwrap();

    match serde_json::from_str(&json) {
        Ok(snippets) => snippets,
        Err(_) => vec![],
    }
}

pub fn add_snippet(
    name: impl Into<String>,
    language: impl Into<String>,
    keyword: impl Into<String>,
    snippet: impl Into<String>,
) {
    let mut snippets = get_snippets();
    let name = name.into();
    let keyword = keyword.into();
    let language = language.into();
    let snippet = snippet.into();

    if snippets.is_empty() {
        snippets.push(Snippet::new(0, name, keyword, language, snippet));
    } else {
        let latest_id = snippets.iter().max_by_key(|s| s.id).unwrap().id;
        snippets.push(Snippet::new(
            latest_id + 1,
            name,
            keyword,
            language,
            snippet,
        ));
    }

    write_snippets(snippets);
}

pub fn delete_snippet(id: usize) {
    let snippets: Vec<Snippet> = get_snippets()
        .iter()
        .map(|s| s.to_owned())
        .filter(|s| s.id != id)
        .collect();

    write_snippets(snippets);
}

pub fn edit_snippet(
    id: usize,
    name: impl Into<String>,
    keyword: impl Into<String>,
    language: impl Into<String>,
    snippet: impl Into<String>,
) {
    let name = name.into();
    let keyword = keyword.into();
    let language = language.into();
    let snippet = snippet.into();

    let mut snippets = Vec::<Snippet>::new();

    for sp in get_snippets() {
        if sp.id == id {
            snippets.push(Snippet::new(
                sp.id,
                name.to_owned(),
                keyword.to_owned(),
                language.to_owned(),
                snippet.to_owned(),
            ));
        } else {
            snippets.push(sp);
        }
    }

    write_snippets(snippets);
}
