use std::path::Path;
use std::{os::unix::process::CommandExt, process::Command};

mod config;
use terminal_colorsaurus::{ColorScheme, QueryOptions, color_scheme};

use clap::{Parser, Subcommand};
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(name = "rod")]
#[command(about = "Terminal background color recognizer", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
    #[arg(long, hide = true)]
    markdown_help: bool,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Print current background type", long_about = None)]
    Print,
    #[command(about = "Global environment from matching the current background", long_about = None)]
    Env {
        #[arg(short, long)]
        no_export: bool,
    },
    #[command(about = "Show example config", long_about = None)]
    Example,
    #[command(arg_required_else_help = true, about = "Run command after extending the arguments given and environment as per settings and current background", long_about = None)]
    Run {
        #[arg(short = 'd')]
        dry: bool,

        #[arg(value_name = "COMMAND")]
        args: Vec<OsString>,
    },
}

macro_rules! utf {
    ($a:expr) => {
        ($a.to_str().expect("Invalid utf-8"))
    };
}

fn main() {
    let bin = Cli::parse();

    #[cfg(debug_assertions)]
    {
        if bin.markdown_help {
            clap_markdown::print_help_markdown::<Cli>();
            return;
        }
    }

    if matches!(bin.command, Commands::Example) {
        println!("{}", config::Config::example());
        return;
    }

    let cfg = config::Config::parse();

    let (bg_is_dark, name) = if ColorScheme::Dark
        == color_scheme(QueryOptions::default()).unwrap_or(if cfg.fallback_to_light {
            ColorScheme::Light
        } else {
            ColorScheme::Dark
        }) {
        (true, "Dark")
    } else {
        (false, "Light")
    };

    let global_env = if bg_is_dark {
        cfg.dark.env
    } else {
        cfg.light.env
    };

    match bin.command {
        Commands::Print => {
            println!("{}", name);
        }
        Commands::Env { no_export } => {
            for (k, v) in global_env {
                if !no_export {
                    print!("export ")
                };
                println!("{k}={v}");
            }
        }
        Commands::Run { dry, args } => {
            let mut command = Command::new(&args[0]);
            command.envs(global_env);

            let command_name = Path::new(&args[0])
                .file_name()
                .expect("Invalid command name")
                .to_str()
                .expect("Command name not utf8");

            if let Some(cmd_conf) = cfg.cmds.get(command_name) {
                let cmd_conf_bg = if bg_is_dark {
                    &cmd_conf.dark
                } else {
                    &cmd_conf.light
                };

                command.args(&cmd_conf_bg.pre_args);
                command.args(&args[1..args.len()]);
                command.args(&cmd_conf_bg.pos_args);
                command.envs(&cmd_conf_bg.env);
            } else {
                command.args(&args[1..args.len()]);
            }
            if dry {
                let envs: Vec<_> = command.get_envs().collect();
                if !envs.is_empty() {
                    print!("env ");
                    for (k, v) in envs {
                        print!("{}={} ", utf!(k), utf!(v.expect("Missing value")));
                    }
                }

                print!("{}", utf!(command.get_program()));
                for a in command.get_args() {
                    print!(" {}", utf!(a));
                }
                println!();
            } else {
                let _ = command.exec();
            }
        }
        Commands::Example => unreachable!("This needs to be handled before"),
    }
}
