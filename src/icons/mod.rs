use std::path::PathBuf;

use whiskers_launcher_rs::api::extensions::get_extension_dir;

use crate::EXTENSION_ID;

pub fn get_icon(name: impl Into<String>) -> String {
    let name = name.into();

    let mut path = get_extension_dir(EXTENSION_ID).unwrap();
    path.push(format!("src/icons/{}", &name));

    path.into_os_string().into_string().unwrap()
}

pub fn get_language_icon(language: &str) -> String {
    let path = match language{
        "other" => get_icon("default.svg"),
        _ => get_icon(format!("{}.svg", language))
    };

    PathBuf::from(path).into_os_string().into_string().unwrap()
}
