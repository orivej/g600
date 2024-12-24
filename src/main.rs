mod profile;
mod profilesio;
mod device;

use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde_yaml;

use crate::profile::Profiles;
use crate::profilesio::{ProfilesIO, ProfilesDump};
use crate::device::G600;

#[derive(Parser)]
struct Cli {
    /// Device path (e.g. /dev/hidraw1). Default: autodetect
    #[arg(long, value_name = "PATH")]
    dev: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print or change active profile
    Active {
        /// Set active profile (0-2)
        #[arg(short, long)]
        profile: Option<u8>,
        /// Set active resolution (0-3)
        #[arg(short, long)]
        resolution: Option<u8>,
    },
    /// Read profiles from the mouse or a binary file and print them to stdout as Yaml
    Read {
        /// Read profiles from the binary file instead of the mouse
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,
        /// Save profiles as a binary file instead of printing them as Yaml
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },
    /// Flash profiles into the mouse or save them in a binary file
    Write {
        /// Read profiles from the binary file instead of Yaml input
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,
        /// Save profiles as a binary file instead of flashing them to the mouse
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Active {
            profile,
            resolution,
        } => {
            let mut dev = G600::open(args.dev).unwrap();
            if profile.is_some() {
                dev.set_active_profile(profile.unwrap()).unwrap();
            }
            if resolution.is_some() {
                dev.set_resolution(resolution.unwrap()).unwrap();
            }
            if profile.is_none() && resolution.is_none() {
                println!("{}", dev.get_active_profile().unwrap());
            }
        }
        Command::Read {
            input,
            output,
        } => {
            let profiles = match input {
                None => G600::open(args.dev).unwrap().read_profiles().unwrap(),
                Some(path) => ProfilesDump::new(&path).read_profiles().unwrap(),
            };
            match output {
                None => print!("{}", serde_yaml::to_string(&profiles).unwrap()),
                Some(path) => ProfilesDump::new(&path).write_profiles(&profiles).unwrap(),
            };
        }
        Command::Write {
            input,
            output,
        } => {
            let profiles = match input {
                None => {
                    let mut input = String::new();
                    std::io::stdin().read_to_string(&mut input).unwrap();
                    let mut profiles: Profiles = serde_yaml::from_str(input.as_str()).unwrap();
                    profiles.fix_ids();
                    profiles.propagate_gshift();
                    profiles
                },
                Some(path) => ProfilesDump::new(&path).read_profiles().unwrap(),
            };
            match output {
                None => G600::open(args.dev).unwrap().write_profiles(&profiles).unwrap(),
                Some(path) => ProfilesDump::new(&path).write_profiles(&profiles).unwrap(),
            }
        }
    }
}
