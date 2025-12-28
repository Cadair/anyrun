use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use glob::glob;
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};
use std::env::var;
use std::path::{Path, PathBuf};

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

    // TODO: Also config this
    let mut store_path = PathBuf::new();
    store_path.push(&var("HOME").unwrap());
    store_path.push(".password-store");
    let base_store_path = store_path.to_path_buf();
    let mut file_glob_pattern = store_path.to_path_buf();
    file_glob_pattern.push("**");
    file_glob_pattern.push("*.gpg");

    let mut matches: RVec<Match> = RVec::new();
    let files = glob(&file_glob_pattern.to_string_lossy()).unwrap();

    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let pattern = Pattern::parse(&input, CaseMatching::Ignore, Normalization::Smart);

    let mut all_files: Vec<String> = Vec::new();
    for entry in files {
        match entry {
            Ok(path) => {
                // Ignore any git files
                if path.starts_with(Path::join(&base_store_path, ".git")) {
                    continue;
                }
                all_files.push(path.to_string_lossy().into_owned());
            }
            Err(e) => println!("{:?}", e),
        }
    }
    let fuzzy_matches: Vec<(String, u32)> = pattern.match_list(all_files, &mut matcher);
    // TODO: Config max n
    for fmatch in fuzzy_matches.iter().take(10) {
        let match_path = Path::new(&fmatch.0);
        let relative_path = match_path.strip_prefix(&base_store_path).unwrap();

        let mut title: RString;
        let description: ROption<RString>;
        if match_path.is_dir() {
            title = RString::from(relative_path.to_string_lossy());
            title.push('/');
            description = ROption::RNone;
        } else {
            title = RString::from(
                relative_path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
            description = ROption::RSome(RString::from(
                relative_path.parent().unwrap().to_string_lossy(),
            ));
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
    matches.into()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    HandleResult::Close
}
