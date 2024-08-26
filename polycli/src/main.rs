use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ConfigSetGetSubcommand, PushSubcommands, RestCommands};
use libpoly::{polyrest::PolyRest, push::{self, MessageLevel}};
use provision::run_provision;
use tmpl::render_alert_template;

mod cli;
mod tmpl;
mod provision;

fn run_cfg_getset(handler: &mut PolyRest, subcommand: ConfigSetGetSubcommand) -> Result<()> {
    match subcommand {
        ConfigSetGetSubcommand::Get { value } => {
            let values = handler.config_get(value)?;
            println!("{:#?}", values);
        },
        ConfigSetGetSubcommand::Set { key, value } => {
            let res = handler.config_set(key, value)?;
            println!("{}", res);
        }
    };

    Ok(())
}

fn run_msg_cmd(username: String, password: String, url: String, subcommand: PushSubcommands, level: MessageLevel) -> anyhow::Result<()> {
    let mut handler = push::PushMessenger::new(username, password, url, true)?;


    let resp = match subcommand {
        PushSubcommands::Alert { header, str_msg } => {
            let rendered = render_alert_template(header, str_msg)?;
            handler.send(level, rendered, push::PushType::HTML)?
        },
        PushSubcommands::Html { msg } => {
            handler.send(level, msg, push::PushType::HTML)?
        },
        PushSubcommands::Cmd { subcommand } => {
            match subcommand {
                cli::PushCmdSubcommands::Dial { number } => {
                    let dial_cmd = format!("tel:\\{}", number);
                    handler.send(level, dial_cmd, push::PushType::Command)?
                },
                cli::PushCmdSubcommands::Key { key } => {
                    let key_cmd = format!("Key:{}", key);
                    handler.send(level, key_cmd, push::PushType::Command)?
                },
                cli::PushCmdSubcommands::Raw { msg } => {
                    handler.send(level, msg, push::PushType::Command)?
                }
            }
        }
    };

    println!("{}", resp);
    Ok(())
}

fn run_rest_cmd(username: String, password: String, url: String, cmd: RestCommands) -> anyhow::Result<()> {
    let mut handler = PolyRest::new(username, password, url, true)?; // TODO: set secure bool from CLI

    match cmd {
        RestCommands::Ctrl => {},
        RestCommands::Mgmt { subcommand } => {
            match subcommand {
                cli::MgmtCommands::Info => {
                    let info = handler.device_info()?;
                    println!("{:#?}", info)
                },
                cli::MgmtCommands::Network => {
                    let info = handler.network_info()?;
                    println!("{:#?}", info)
                }, 
                cli::MgmtCommands::NetStats => {
                    let stats = handler.network_stats()?;
                    println!("{:#?}", stats);
                }, 
                cli::MgmtCommands::Config { subcommand } => {
                    run_cfg_getset(&mut handler, subcommand)?;
                },
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Rest{subcommand} => {
            run_rest_cmd(args.user, args.pass, args.url, subcommand)?; 
        }, 
        Commands::Push { subcommand, level } => {
            run_msg_cmd(args.user, args.pass, args.url, subcommand, level)?;
        },
        Commands::Provisioner { port, path } => {
            run_provision(format!("0.0.0.0:{}", port), path).await?;
        }
    };

    Ok(())
}
