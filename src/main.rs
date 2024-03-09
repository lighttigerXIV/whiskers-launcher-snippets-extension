mod actions;
mod icons;
mod results;
mod snippets;

use actions::handle_actions;
use results::handle_results;
use snippets::init_snippets;
use whiskers_launcher_rs::api::{self, extensions::get_extension_context};

const EXTENSION_ID: &str = "lighttigerxiv/snippets";

fn main() {
    let context = get_extension_context().unwrap();

    init_snippets();

    match context.action {
        api::extensions::Action::GetResults => handle_results(context.to_owned()),
        api::extensions::Action::RunAction => handle_actions(context.to_owned()),
    }
}
