use clap::{Parser, Subcommand};
use hidraw::{Device, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use static_assertions::const_assert_eq;
use std::fmt;
use std::io::Read;

const NUM_PROFILES: usize = 3;
const NUM_DPI: usize = 4;
// Buttons (as labeled on the mouse):
// 1 2 6: index, middle, ring finger buttons
// 3 4 5: wheel down, left, right
// 8 7: below wheel
// 9-20: thumb buttons
const NUM_BUTTONS: usize = 20;
// const DPI_MIN: u16 = 200;
// const DPI_MAX: u16 = 8200;

const ACTIVE_PROFILE_REPORT_ID: u8 = 0xF0;
const PROFILE_REPORT_ID: [u8; NUM_PROFILES] = [0xF3, 0xF4, 0xF5];

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum LedEffect {
    Solid,
    Breath,
    Cycle,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum ReportRate {
    Hz1000,
    Hz500,
    Hz333,
    Hz250,
    Hz200,
    Hz166,
    Hz142,
    Hz125,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
enum ButtonAction {
    #[default]
    Key,
    LeftClick,
    RightClick,
    WheelClick,
    WheelLeft,
    WheelRight,
    M10,
    M11,
    M12,
    M13,
    M14,
    M15,
    M16,
    M17,
    M18,
    M19,
    M20,
    DPIUp = 0x11,    // dpis: 0 -> 1 -> 2 -> 3 -> 3
    DPIDown,         // dpis: 3 -> 2 -> 1 -> 0 -> 0
    DPICycle,        // dpis: 0 -> 1 -> 2 -> 3 -> 0
    ProfileCycle,    // Default for G8 in all profiles
    DPIShift,        // dpi = dpi_shift while DPIShift button is pressed.
    DPIDefault,      // dpis: -> dpi_default
    GShift = 0x17,   // Default for ring finger (G6) in the first two profiles
    M11a,            // Same effect as M11
    M12a,            // Same effect as M12
    X1A,
    X1B,
    X1C,
}

impl ButtonAction {
    fn is_default(&self) -> bool {
        match *self {
            ButtonAction::Key => true,
            _ => false,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum KeyModifier {
    None = 0,
    Ctrl = 0x01,
    Shift = 0x02,
    Alt = 0x04,
    Meta = 0x08,
    RCtrl = 0x10,
    RShift = 0x20,
    RAlt = 0x40,
    RMeta = 0x80,
}

fn is_zero(x: &u8) -> bool {
    *x == 0
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Button {
    #[serde(skip_serializing_if = "ButtonAction::is_default", default)]
    action: ButtonAction,
    #[serde(skip_serializing_if = "is_zero", default)]
    modifiers: u8,
    #[serde(skip_serializing_if = "is_zero", default)]
    key: u8,
}

#[repr(C, packed)]
#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    #[serde(skip_serializing, default)]
    id: u8,
    led_color: Color,
    led_effect: LedEffect,
    led_duration: u8, // 0 to 15 seconds
    #[serde(skip_serializing, default)]
    unknown1: [u8; 5],
    report_rate: ReportRate,
    dpi_shift: u8,       // dpi = value * 50; dpi is between 200 and 8200; 0 is disabled
    dpi_default: u8,     // between 1 and 4
    dpis: [u8; NUM_DPI], // dpi = value * 50; dpi is between 200 and 8200; 0 is disabled
    #[serde(skip_serializing, default)]
    unknown2: [u8; 13],
    buttons: [Button; NUM_BUTTONS],
    g_led_color: Color,
    g_buttons: [Button; NUM_BUTTONS],
}

const_assert_eq!(std::mem::size_of::<Profile>(), 154);

impl Profile {
    fn propagate_gshift(&mut self) {
        for (i, button) in self.buttons.iter_mut().enumerate() {
            match button.action {
                ButtonAction::GShift => {
                    self.g_buttons[i].action = ButtonAction::GShift;
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Profiles {
    profiles: [Profile; NUM_PROFILES],
}

impl Profiles {
    fn propagate_gshift(&mut self) {
        for profile in self.profiles.iter_mut() {
            profile.propagate_gshift();
        }
    }
}

#[repr(C, packed)]
struct ActiveProfile {
    id: u8,
    info: u8, // unused:1, resolution:2, unused:1, profile:4
    unused: u16,
}

impl ActiveProfile {
    fn profile(&self) -> u8 {
        self.info >> 4
    }

    fn resolution(&self) -> u8 {
        (self.info >> 1) & 0x03
    }
}

impl fmt::Display for ActiveProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ActiveProfile {{ profile: {}, resolution: {} }}",
            self.profile(),
            self.resolution()
        )
    }
}

fn get_active_profile_report(dev: &mut hidraw::Device) -> Result<ActiveProfile> {
    dev.get_feature_report::<ActiveProfile>(ACTIVE_PROFILE_REPORT_ID)
}

fn read_profile(dev: &mut hidraw::Device, id: u8) -> Result<Profile> {
    dev.get_feature_report::<Profile>(id)
}

fn read_profiles(dev: &mut hidraw::Device) -> Result<Profiles> {
    let mut profiles = Vec::new();
    for i in 0..NUM_PROFILES {
        profiles.push(read_profile(dev, PROFILE_REPORT_ID[i])?)
    }
    Ok(Profiles {
        profiles: profiles.try_into().unwrap(),
    })
}

fn write_profile(dev: &mut hidraw::Device, id: u8, profile: &mut Profile) -> Result<()> {
    profile.id = id;
    dev.send_feature_report::<Profile>(profile)
}

fn write_profiles(dev: &mut hidraw::Device, profiles: &mut Profiles) -> Result<()> {
    for i in 0..NUM_PROFILES {
        write_profile(dev, PROFILE_REPORT_ID[i], &mut profiles.profiles[i])?
    }
    Ok(())
}

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
