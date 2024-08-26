use clap::{Parser, Subcommand};
use libpoly::push::MessageLevel;


#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Set the username for all queries
    #[arg(short, long, env="POLY_USER", default_value= "Polycom")]
    pub user: String,
    // Set the password for all queries
    #[arg(short, long, env="POLY_PASS")]
    pub pass: String,

    /// Set the device URL for all queries
    #[arg(long, env="POLY_URL")]
    pub url: String,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run REST commands
    Rest{
        #[clap(subcommand)]
        subcommand: RestCommands
    },

    /// Send a push notification to the device
    Push {
        /// Message priority level
        #[arg(long="level", short='l', default_value_t = MessageLevel::Critical, value_parser = clap::builder::EnumValueParser::<MessageLevel>::new())]
        level: MessageLevel,

        /// Type of push event to send
        #[clap(subcommand)]
        subcommand: PushSubcommands
    },

    /// Start up an HTTP file server server pointing to the specified path. If the Polycom device is configured to look for this server, it 
    /// will use the config files at the given path to configure the device.
    Provisioner {

        /// The port to expose the service at
        #[arg(long="port", short='p', default_value_t=8000)]
        port: u32,

        /// The path to run the file server
        path: String
    }

}

#[derive(Debug, Subcommand)]
pub enum PushSubcommands {
    /// Send a command to the device
    Cmd {
        #[clap(subcommand)]
        subcommand: PushCmdSubcommands
    },
    /// Send an HTML command to the device
    Html {
        /// A raw message to send
        msg: String
    },
    /// Send a pre-designed HTML alert message to the device
    Alert {
        /// A header for the HTML alert
        header: String,
        /// A string message to send
        str_msg: String,
    }
}

#[derive(Debug, Subcommand)]
pub enum PushCmdSubcommands {
    /// Send a raw command string to the device
    Raw {
        /// String to send in the Data XML body
        msg: String
    },
    /// Activate a key on the device
    Key{
        key: String
    },
    /// Dial a phone number
    Dial{
        number: String
    }
}


#[derive(Debug, Subcommand)]
pub enum RestCommands {
    /// Run management REST commands
    Mgmt{
        #[clap(subcommand)]
        subcommand: MgmtCommands
    },

    Ctrl
}

#[derive(Debug, Subcommand)]
pub enum MgmtCommands{
    /// Print Device Info
    Info,
    /// Print network counters
    Network,
    /// Print network stats
    NetStats,
    /// Print device config
    Config {
        #[clap(subcommand)]
        subcommand: ConfigSetGetSubcommand
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigSetGetSubcommand {
    /// Get a config value
    Get {
        #[arg(value_name="CONFIG_VAL")]
        value: String
    },
    Set {
        #[arg(value_name="CONFIG_KEY")]
        key: String,

        #[arg(value_name="CONFIG_VALUE")]
        value: String
    }
}
