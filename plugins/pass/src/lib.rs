use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use glob::glob;
use std::env::var;
use std::path::{PathBuf, Path};

#[init]
fn init(_config_dir: RString) {
    eprintln!("Hello from pass plugin");
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Pass".into(),
        icon: "lock".into(),
    }
}

#[get_matches]
fn get_matches(input: RString) -> RVec<Match> {
    // TODO: Config this
    if input.len() < 2 {
        return RVec::new();
    }

    // Also config thisuse std::path::Path;
    let mut store_path = PathBuf::new();
    store_path.push(&var("HOME").unwrap());
    store_path.push(".password-store");
    let base_store_path = store_path.to_path_buf();
    let mut file_glob_pattern = store_path.to_path_buf();
    file_glob_pattern.push("**");
    file_glob_pattern.push(format!("*{}*", input));
    let mut dir_glob_pattern = store_path.to_path_buf();
    dir_glob_pattern.push("**");
    dir_glob_pattern.push(format!("*{}*", input));
    dir_glob_pattern.push("**");

    let mut matches: RVec<Match> = RVec::new();
    let files = glob(&file_glob_pattern.to_string_lossy()).unwrap();
    let dirs = glob(&dir_glob_pattern.to_string_lossy()).unwrap();

    for entry in  dirs.chain(files) {
        match entry {
            Ok(path) => {
                // Ignore any git files
                if path.starts_with(Path::join(&base_store_path, ".git")) {
                    continue
                }
                let relative_path = path.strip_prefix(&base_store_path).unwrap();
                let title:RString;
                let description:ROption<RString>;
                if path.is_dir() {
                    title = RString::from(relative_path.to_string_lossy());
                    description = ROption::RNone;
                } else {
                    title = RString::from(relative_path.file_stem().unwrap().to_string_lossy().into_owned());
                    description =  ROption::RSome(RString::from(relative_path.parent().unwrap().to_string_lossy()));
                }
                // let filename = RString::from(without_ext.to_string_lossy().into_owned());
                matches.push(Match {
                    title: title,
                    description: description,
                    use_pango: false,
                    id: ROption::RNone,
                    icon: ROption::RNone,
                });
            }
            Err(e) => println!("{:?}", e),
        }
    }
    matches.into()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    HandleResult::Close
}
