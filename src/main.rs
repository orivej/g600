mod device;
mod profile;

use clap::{Parser, Subcommand};
use hidraw::Device;
use serde_yaml;
use std::io::Read;

use device::{get_active_profile_report, read_profiles, write_profiles};
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
    /// Print active profile
    Active {},
    /// Read profiles from the mouse and print them to stdout as Yaml
    Read {},
    /// Parse profiles from stdin as Yaml and write them to the mouse
    Write {},
}

fn main() {
    let args = Cli::parse();

    let mut dev = Device::open(args.dev).unwrap();

    match args.command {
        Command::Active {} => {
            let apr = get_active_profile_report(&mut dev).unwrap();
            println!("{apr}");
        }
        Command::Read {} => {
            let profiles = read_profiles(&mut dev).unwrap();
            print!("{}", serde_yaml::to_string(&profiles).unwrap());
        }
        Command::Write {} => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let mut profiles: Profiles = serde_yaml::from_str(input.as_str()).unwrap();
            profiles.propagate_gshift();
            write_profiles(&mut dev, &mut profiles).unwrap();
        }
    }
}
