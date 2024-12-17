mod device;
mod profile;

use clap::{Parser, Subcommand};
use serde_yaml;
use std::io::Read;

use device::G600;
use profile::Profiles;

#[derive(Parser)]
struct Cli {
    /// Device path
    #[arg(long, value_name = "PATH", default_value = "/dev/hidraw1")]
    dev: String,
    /// Print internal representation of profiles
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
    /// Read profiles from the mouse and print them to stdout as Yaml
    Read {},
    /// Parse profiles from stdin as Yaml and write them to the mouse
    Write {},
}

fn main() {
    let args = Cli::parse();

    let mut dev = G600::open(args.dev).unwrap();

    match args.command {
        Command::Active { profile, resolution } => {
            if profile.is_some() {
                dev.set_active_profile(profile.unwrap()).unwrap()
            }
            if resolution.is_some() {
                dev.set_resolution(resolution.unwrap()).unwrap()
            }
            if profile.is_none() && resolution.is_none() {
                println!("{}", dev.get_active_profile().unwrap());
            }
        }
        Command::Read {} => {
            let profiles = dev.read_profiles().unwrap();
            print!("{}", serde_yaml::to_string(&profiles).unwrap());
        }
        Command::Write {} => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let mut profiles: Profiles = serde_yaml::from_str(input.as_str()).unwrap();
            profiles.propagate_gshift();
            dev.write_profiles(&mut profiles).unwrap();
        }
    }
}
