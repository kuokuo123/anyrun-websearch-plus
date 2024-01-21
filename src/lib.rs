use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::{Deserialize, Serialize};
use std::{fs, process::Command};
use urlencoding::encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Engine {
    Google,
    Custom { name: String, url: String, secondary_prefix: String },
}

impl Engine {
    fn value(&self) -> &str {
        match self {
            Self::Google => "google.com/search?q={}",
            Self::Custom { url, .. } => url,
        }
    }
    fn secondary_prefix(&self) -> &str {
        match self {
            Self::Google => "",
            Self::Custom { secondary_prefix, .. } => secondary_prefix,
        }
    }
    fn name(&self) -> &str {
        match self {
            Self::Google => "Google",
            Self::Custom { name, .. } => name,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    prefix: String,
    engines: Vec<Engine>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: "?".to_string(),
            engines: vec![Engine::Google],
        }
    }
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/websearch_plus.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Websearch+".into(),
        icon: "distributor-logo-netrunner".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, config: &Config) -> RVec<Match> {
    if !input.starts_with(&config.prefix) {
        RVec::new()
    } else {
        config
            .engines
            .iter()
            .filter(|engine| input.strip_prefix(&config.prefix)
                    .expect("Unable to strip prefix from input lines")
                    .starts_with(&engine.secondary_prefix())
            )
            .enumerate()
            .map(|(_, engine)| Match {
                title: input.trim_start_matches(&config.prefix).trim_start_matches(&engine.secondary_prefix()).into(),
                description: ROption::RSome(format!("Search with {}", engine.name()).into()),
                use_pango: false,
                icon: ROption::RNone,
                id: ROption::RNone,
            })
            .collect()
    }
}

#[handler]
fn handler(selection: Match, config: &Config) -> HandleResult {

    let engine = config
        .engines
        .iter()
        .find(|engine| engine.name() == selection.description.clone().unwrap().replace("Search with ", ""))
        .unwrap();

    if let Err(why) = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "xdg-open https://{}",
            engine
                .value()
                .replace("{}", &encode(&selection.title.to_string()))
        ))
        .spawn()
    {
        println!("Failed to perform websearch-plus: {}", why);
    }

    HandleResult::Close
}
