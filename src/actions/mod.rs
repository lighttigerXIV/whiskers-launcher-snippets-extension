use std::process::exit;

use whiskers_launcher_rs::{api::extensions::{get_extension_dialog_response, Context}, others::send_notification};

use crate::snippets::{add_snippet, delete_snippet, edit_snippet};

pub fn handle_actions(context: Context) {
    let action = context.extension_action.unwrap();

    if action == "add_snippet" {
        let response = get_extension_dialog_response().unwrap();
        let name = response.get_result_value("name").unwrap();
        let keyword = response.get_result_value("keyword").unwrap();
        let language = response.get_result_value("language").unwrap();
        let snippet = response.get_result_value("snippet").unwrap();

        if name.trim().is_empty() || keyword.trim().is_empty() || snippet.trim().is_empty(){
            send_notification("Empty Fields", "Can't have empty fields");
            exit(0)
        }

        add_snippet(name, language, keyword, snippet);
    }

    if action == "edit_snippet" {
        let response = get_extension_dialog_response().unwrap();
        let name = response.get_result_value("name").unwrap();
        let keyword = response.get_result_value("keyword").unwrap();
        let language = response.get_result_value("language").unwrap();
        let snippet = response.get_result_value("snippet").unwrap();
        let id: usize = response.args.unwrap().to_owned()[0].parse().unwrap();

        edit_snippet(id, name, keyword, language, snippet);
    }

    if action == "delete_snippet" {
        let id: usize = context.custom_args[0].to_owned().parse().unwrap();

        delete_snippet(id);
    }
}
