use anyhow::Error;
use anyhow::Result;
use clap::{Parser, Subcommand};
use config::write_config;
use config::Configuration;

use crate::models::monitor::Monitor;

mod config;
mod models;
mod postgres_client;
mod utility;

#[derive(Parser, Debug)]
#[command(name = "pgm")]
#[command(author = "Todd Martin <tcmart14@posteo.net")]
#[command(version = "1.0")]
#[command(about = "Query pg_auto_failover monitor", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // add pg_auto_failover monitor
    Add {
        #[clap(long)]
        host: Option<String>,

        #[clap(long)]
        port: Option<i32>,

        #[clap(long)]
        user: Option<String>,

        #[clap(long)]
        password: Option<String>,

        #[clap(long)]
        database_name: Option<String>,
    },

    // list databases in config
    List,

    // Commands on monitor
    Show {
        #[arg(long)]
        host: String,

        #[arg(long)]
        state: bool,
    },
}

fn handle_add_database(config: &mut Configuration, monitor: Monitor) -> Result<(), Error> {
    config.add_monitor(monitor);
    write_config(&config)?;
    Ok(())
}

fn handle_list(config: Configuration) -> Result<(), Error> {
    for database in config.monitors.into_iter() {
        println!("{}", database);
    }
    Ok(())
}

fn handle_show_state(config: Configuration, host: &str) -> Result<(), Error> {
    let monitor = config
        .monitors
        .iter()
        .find(|m| m.host == Some(host.to_string()));

    if let Some(monitor) = monitor {
        postgres_client::client::show_state(monitor.clone())?;
        return Ok(());
    } else {
        Err(Error::msg("Host not found"))
    }
}

fn main() -> Result<(), Error> {
    let mut config = config::get_config()?;
    let args = CLI::parse();

    let command = match args.command {
        Some(c) => c,
        None => {
            std::process::exit(0);
        }
    };

    // print!("{:#?}", command);

    match command {
        Commands::Add {
            host,
            port,
            user,
            password,
            database_name,
        } => {
            let monitor = Monitor {
                host,
                port,
                user,
                password,
                database_name,
            };
            handle_add_database(&mut config, monitor)?;
        }
        Commands::List => handle_list(config)?,
        Commands::Show { host, state } => {
            if state == true {
                handle_show_state(config, host.as_str())?;
            }
        }
    }

    Ok(())
}
