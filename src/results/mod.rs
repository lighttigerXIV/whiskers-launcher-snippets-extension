use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use whiskers_launcher_rs::{
    actions::{self, Action},
    api::extensions::{send_extension_results, Context},
    dialog::{self, DialogField, SelectField},
    results::{self, WhiskersResult},
    utils::{get_search, Search},
};

use crate::{
    icons::{get_icon, get_language_icon},
    snippets::get_snippets,
    EXTENSION_ID,
};

pub fn handle_results(context: Context) {
    let search = get_search(context.search_text.unwrap());

    if search.keyword.is_none() && search.search_text.is_empty() {
        show_default_results();
    }

    if search.keyword.is_some() {
        let kwd = search.keyword.to_owned().unwrap();

        if kwd == "e" || kwd == "edit" {
            show_edit_results(search.to_owned());
        }

        if kwd == "d" || kwd == "delete" {
            show_delete_results(search.to_owned());
        }

        show_snippets_results(search.to_owned());
    }

    if search.keyword.is_none() && !search.search_text.trim().is_empty() {
        show_snippets_results(search.to_owned());
    }

    send_extension_results(vec![]);
}

fn show_default_results() {
    let mut results = Vec::<WhiskersResult>::new();

    let name_field = DialogField::Input(
        dialog::Input::new("name", "Name", "")
            .placeholder("Name")
            .description("The snippet name. Example: 'Mutable State Flow'"),
    );

    let keyword_field = DialogField::Input(
        dialog::Input::new("keyword", "Keyword", "")
            .placeholder("Keyword")
            .description("The snippet keyword. Used to search"),
    );

    let mut languages: Vec<SelectField> = vec![];
    languages.push(SelectField::new("other", "Other"));
    languages.push(SelectField::new("c-sharp", "C#"));
    languages.push(SelectField::new("go", "GO"));
    languages.push(SelectField::new("java", "Java"));
    languages.push(SelectField::new("js", "JavaScript"));
    languages.push(SelectField::new("kotlin", "Kotlin"));
    languages.push(SelectField::new("python", "Python"));
    languages.push(SelectField::new("rust", "Rust"));
    languages.push(SelectField::new("ts", "Typescript"));

    let language_field = DialogField::Select(
        dialog::Select::new("language", "Language", "other", languages)
            .description("The snippet programming language"),
    );

    let snippet_field = DialogField::TextArea(
        dialog::TextArea::new("snippet", "Snippet", "")
            .description("The snippet. If needed use {%0}, {%1} and so on as paremeter")
            .placeholder("Snippet"),
    );

    let add_snippet_fields = vec![name_field, keyword_field, language_field, snippet_field];

    let add_snippet_action = Action::Dialog(
        actions::Dialog::new(
            EXTENSION_ID,
            "Add Snippet",
            "add_snippet",
            add_snippet_fields,
        )
        .primary_button_text("Add"),
    );

    results.push(WhiskersResult::Text(
        results::Text::new("Add Snippet", add_snippet_action)
            .icon(get_icon("plus.svg"))
            .tint_icon(true),
    ));

    send_extension_results(results);
}

fn show_snippets_results(search: Search) {
    let snippets = get_snippets();
    let mut results = Vec::<WhiskersResult>::new();
    let matcher = SkimMatcherV2::default();

    for snippet in snippets {
        if search.keyword.is_some() {
            let keyword = search.keyword.to_owned().unwrap();

            if matcher.fuzzy_match(&snippet.keyword, &keyword).is_some() {
                let search_split = search.search_text.split_whitespace();
                let mut replaced_snippet = snippet.snippet.to_owned().to_string();

                for (index, word) in search_split.enumerate() {
                    let pattern = format!("{{%{}}}", index);
                    replaced_snippet = replaced_snippet.replace(&pattern, word);
                }

                results.push(WhiskersResult::TitleAndText(
                    results::TitleAndText::new(
                        format!("{} - {}", &snippet.name, &snippet.keyword),
                        &replaced_snippet,
                        actions::Action::CopyToClipboard(actions::CopyToClipboard::new(
                            &replaced_snippet,
                        )),
                    )
                    .icon(get_language_icon(&snippet.languague))
                    .tint_icon(true),
                ))
            }
        } else {
            if matcher
                .fuzzy_match(&snippet.keyword, &search.search_text)
                .is_some()
            {
                results.push(WhiskersResult::TitleAndText(
                    results::TitleAndText::new(
                        format!("{} - {}", &snippet.name, &snippet.keyword),
                        &snippet.snippet,
                        actions::Action::CopyToClipboard(actions::CopyToClipboard::new(
                            &snippet.snippet,
                        )),
                    )
                    .icon(get_language_icon(&snippet.languague))
                    .tint_icon(true),
                ))
            }
        }
    }

    send_extension_results(results);
}

fn show_edit_results(search: Search) {
    let mut results = Vec::<WhiskersResult>::new();
    let snippets = get_snippets();
    let matcher = SkimMatcherV2::default();

    for snippet in snippets {
        let search_string = format!("{}{}", &snippet.keyword, &snippet.name);

        if matcher
            .fuzzy_match(&search_string, &search.search_text)
            .is_some()
        {
            let name_field = DialogField::Input(
                dialog::Input::new("name", "Name", &snippet.name)
                    .placeholder("Name")
                    .description("The snippet name. Example: 'Mutable State Flow'"),
            );

            let keyword_field = DialogField::Input(
                dialog::Input::new("keyword", "Keyword", &snippet.keyword)
                    .placeholder("Keyword")
                    .description("The snippet keyword. Used to search"),
            );

            let mut languages: Vec<SelectField> = vec![];
            languages.push(SelectField::new("other", "Other"));
            languages.push(SelectField::new("c-sharp", "C#"));
            languages.push(SelectField::new("go", "GO"));
            languages.push(SelectField::new("java", "Java"));
            languages.push(SelectField::new("js", "JavaScript"));
            languages.push(SelectField::new("kotlin", "Kotlin"));
            languages.push(SelectField::new("python", "Python"));
            languages.push(SelectField::new("rust", "Rust"));
            languages.push(SelectField::new("ts", "Typescript"));

            let language_field = DialogField::Select(
                dialog::Select::new("language", "Language", &snippet.languague, languages)
                    .description("The snippet programming language"),
            );

            let snippet_field = DialogField::TextArea(
                dialog::TextArea::new("snippet", "Snippet", &snippet.snippet)
                    .description("The snippet. If needed use {%0}, {%1} and so on as paremeter")
                    .placeholder("Snippet"),
            );

            let edit_snippet_fields =
                vec![name_field, keyword_field, language_field, snippet_field];

            let add_snippet_action = Action::Dialog(
                actions::Dialog::new(
                    EXTENSION_ID,
                    format!("Edit {}", &snippet.name),
                    "edit_snippet",
                    edit_snippet_fields,
                )
                .args(vec![snippet.id.to_string()])
                .primary_button_text("Save"),
            );

            results.push(WhiskersResult::Text(
                results::Text::new(
                    format!(
                        "Edit ({}) ({}) ({})",
                        &snippet.name, &snippet.keyword, &snippet.languague
                    ),
                    add_snippet_action,
                )
                .icon(get_icon("pencil.svg"))
                .tint_icon(true),
            ));
        }
    }

    send_extension_results(results);
}

fn show_delete_results(search: Search) {
    let snippets = get_snippets();
    let mut results = Vec::<WhiskersResult>::new();
    let matcher = SkimMatcherV2::default();

    for snippet in snippets {
        if matcher
            .fuzzy_match(&snippet.keyword, &search.search_text)
            .is_some()
        {
            results.push(WhiskersResult::TitleAndText(
                results::TitleAndText::new(
                    format!(
                        "Delete ({}) ({}) ({})",
                        &snippet.name, &snippet.keyword, &snippet.languague
                    ),
                    &snippet.snippet,
                    actions::Action::Extension(
                        actions::Extension::new(EXTENSION_ID, "delete_snippet")
                            .args(vec![snippet.id.to_string()]),
                    ),
                )
                .icon(get_icon("trash.svg"))
                .tint_icon(true),
            ))
        }
    }

    send_extension_results(results);
}
