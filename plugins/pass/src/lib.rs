use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use glob::glob;

#[init]
fn init(_config_dir: RString) {
    println!("Hello from pass plugin");
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
    let mut matches: RVec<Match> = RVec::new();
    let glob_pattern = format!("~/.password-store/**/{}.gpg", &input);
    for entry in glob(&glob_pattern).unwrap() {
        match entry {
            Ok(path) => {
                let filename = RString::from(path.file_name().unwrap().to_string_lossy().into_owned());
                matches.push(Match{
                    title: filename,
                    description: ROption::RNone,
                    use_pango: false,
                    id: ROption::RNone,
                    icon: ROption::RNone,
                });
            }
            Err(e) => println!("{:?}", e),
        }
    }
    matches
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    HandleResult::Copy(selection.title.into_bytes())
}
