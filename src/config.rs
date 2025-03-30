use std::{collections::HashMap, fs::create_dir_all, fs::read_to_string, fs::write};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ThemeConf {
    #[serde(default)]
    pub(crate) env: HashMap<String, String>,
    #[serde(default)]
    pub(crate) pre_args: Vec<String>,
    #[serde(default)]
    pub(crate) pos_args: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommandThemes {
    #[serde(default)]
    pub(crate) dark: ThemeConf,
    #[serde(default)]
    pub(crate) light: ThemeConf,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct GlobalTheme {
    #[serde(default)]
    pub(crate) env: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub(crate) dark: GlobalTheme,
    #[serde(default)]
    pub(crate) light: GlobalTheme,
    #[serde(default)]
    pub(crate) cmds: HashMap<String, CommandThemes>,
    #[serde(default)]
    pub(crate) fallback_to_light: bool,
}

impl Config {
    pub(crate) fn parse() -> Config {
        let config_path = dirs::config_dir()
            .expect("Config Path doesn't exist")
            .join("rod")
            .join("config.toml");
        let config_str = read_to_string(&config_path).unwrap_or("".to_string());
        let res: Config = toml::from_str(&config_str).expect("Config error");

        if config_str.is_empty() {
            create_dir_all(config_path.parent().unwrap()).expect("Couldn't create config dir");
            write(
                &config_path,
                toml::to_string(&res).expect("Could not serialize"),
            )
            .expect("Write failed");
        }
        res
    }
    pub(crate) fn example() -> String {
        let cfg = Config {
            fallback_to_light: false,
            dark: GlobalTheme {
                env: HashMap::from_iter([("THEME".to_string(), "dark".to_string())]),
            },

            light: GlobalTheme {
                env: HashMap::from_iter([("THEME".to_string(), "light".to_string())]),
            },
            cmds: HashMap::from_iter([
                (
                    "test_cmd".to_string(),
                    CommandThemes {
                        dark: ThemeConf {
                            env: HashMap::from_iter([("COLOR".to_string(), "dark".to_string())]),
                            pre_args: vec![],
                            pos_args: vec!["--color=dark".to_string()],
                        },
                        light: ThemeConf {
                            env: HashMap::from_iter([("COLOR".to_string(), "light".to_string())]),
                            pre_args: vec![],
                            pos_args: vec!["--color=dark".to_string()],
                        },
                    },
                ),
                (
                    "fzf".to_string(),
                    CommandThemes {
                        dark: ThemeConf {
                            env: HashMap::new(),
                            pre_args: vec!["--color=dark".to_string()],
                            pos_args: vec![],
                        },
                        light: ThemeConf {
                            env: HashMap::new(),
                            pre_args: vec!["--color=light".to_string()],
                            pos_args: vec![],
                        },
                    },
                ),
            ]),
        };
        toml::to_string(&cfg).unwrap()
    }
}
