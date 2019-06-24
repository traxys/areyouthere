extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate simplelog;

#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    message: String,
    id: Option<String>,
}

fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generates completion scripts for your shell")
                .arg(
                    Arg::with_name("SHELL")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh"])
                        .help("The shell to generate the script for"),
                ),
        )
        .arg(
            Arg::with_name("PORT")
                .help("Port on which to listen")
                .short("p")
                .long("port")
                .default_value("1526"),
        )
        .arg(
            Arg::with_name("ID")
                .help("Computer ID")
                .short("i")
                .long("id")
                .takes_value(true),
        )
        .arg(Arg::with_name("VERBOSE").short("v"))
}

fn main() {
    let matches = build_cli().get_matches();

    let log_level = if matches.is_present("VERBOSE") {
        simplelog::LevelFilter::Debug
    } else {
        simplelog::LevelFilter::Info
    };

    if let Err(_) = simplelog::TermLogger::init(
        log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    ) {
        simplelog::SimpleLogger::init(log_level, simplelog::Config::default()).unwrap();
    }

    let info = Info {
        message: "How do you do ?".to_string(),
        id: matches.value_of("ID").map(|s| s.to_string()),
    };

    match matches.subcommand() {
        ("completions", Some(sub_matches)) => {
            let shell = sub_matches.value_of("SHELL").unwrap();
            build_cli().gen_completions_to(
                "areyoualive",
                shell.parse().unwrap(),
                &mut std::io::stdout(),
            );
        }
        _ => {
            let listener = match std::net::TcpListener::bind(format!(
                "0.0.0.0:{}",
                matches.value_of("PORT").unwrap()
            )) {
                Ok(l) => l,
                Err(e) => {
                    error!("Could not listen: {}", e);
                    return;
                }
            };
            info!("Listening on {}", matches.value_of("PORT").unwrap());
            for conn in listener.incoming() {
                match conn {
                    Ok(conn) => {
                        if let Err(e) = serde_json::to_writer(&conn, &info) {
                            warn!("Error sending data: {}", e);
                        }
                    }
                    Err(e) => warn!("Error accepting connection: {}", e),
                }
            }
        }
    }
}
