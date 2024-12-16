use hidraw::{Device, Result};
use serde::Serialize;
use serde_yaml;
use static_assertions::const_assert_eq;

const NUM_PROFILES: usize = 3;
const NUM_DPI: usize = 4;
// const DPI_MIN: u16 = 200;
// const DPI_MAX: u16 = 8200;

const ACTIVE_PROFILE_REPORT_ID: u8 = 0xF0;
const PROFILE_REPORT_ID: [u8; NUM_PROFILES] = [0xF3, 0xF4, 0xF5];

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Serialize)]
enum LedEffect {
    #[default]
    Solid,
    Breath,
    Cycle,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Serialize)]
enum ReportRate {
    #[default]
    Hz1000,
    Hz500,
    Hz333,
    Hz250,
    Hz200,
    Hz166,
    Hz142,
    Hz125,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Serialize)]
enum ButtonAction {
    #[default]
    Key,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    IncResolution = 0x11,
    DecResolution,
    NextResolution,
    NextProfile,
    AlternateResolution,
    SecondMode = 0x17,
}

impl ButtonAction {
    fn is_default(&self) -> bool {
        match *self {
            ButtonAction::Key => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Serialize)]
enum KeyModifier {
    #[default]
    None = 0,
    LeftCtrl = 0x01,
    LeftShift = 0x02,
    LeftAlt = 0x04,
    LeftMeta = 0x08,
    RightCtrl = 0x10,
    RightShift = 0x20,
    RightAlt = 0x40,
    RightMeta = 0x80,
}

fn is_zero(x: &u8) -> bool {
    *x == 0
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, Serialize)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, Serialize)]
struct Button {
    #[serde(skip_serializing_if = "ButtonAction::is_default")]
    action: ButtonAction,
    #[serde(skip_serializing_if = "is_zero")]
    modifiers: u8,
    #[serde(skip_serializing_if = "is_zero")]
    key: u8,
}

#[repr(C, packed)]
#[derive(Debug, Default, Serialize)]
struct Profile {
    #[serde(skip_serializing)]
    id: u8,
    led_color: Color,
    led_effect: LedEffect,
    led_duration: u8, // 0 to 15 seconds
    #[serde(skip_serializing)]
    unknown1: [u8; 5],
    report_rate: ReportRate,
    g_dpi: u8,           // dpi = value * 50; dpi is between 200 and 8200; 0 is disabled
    dpi_default: u8,     // between 1 and 4
    dpis: [u8; NUM_DPI], // dpi = value * 50; dpi is between 200 and 8200; 0 is disabled
    #[serde(skip_serializing)]
    unknown2: [u8; 13],
    buttons: [Button; 20],
    g_led_color: Color,
    g_buttons: [Button; 20],
}

const_assert_eq!(std::mem::size_of::<Profile>(), 154);

#[derive(Debug, Default, Serialize)]
struct Profiles {
    profiles: [Profile; NUM_PROFILES],
}

#[repr(C, packed)]
#[derive(Debug)]
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

fn get_active_profile_report(dev: &mut hidraw::Device) -> Result<ActiveProfile> {
    dev.get_feature_report::<ActiveProfile>(ACTIVE_PROFILE_REPORT_ID)
}

fn read_profile(dev: &mut hidraw::Device, id: u8) -> Result<Profile> {
    dev.get_feature_report::<Profile>(id)
}

fn read_profiles(dev: &mut hidraw::Device) -> Result<Profiles> {
    let mut profiles: Profiles = Default::default();
    for i in 0..NUM_PROFILES {
        profiles.profiles[i] = match read_profile(dev, PROFILE_REPORT_ID[i]) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(profiles)
}

fn main() {
    let mut dev = Device::open("/dev/hidraw2").unwrap();
    // let apr = get_active_profile_report(&mut dev).unwrap();
    // println!("{apr:?}");
    // println!("profile = {:?}", apr.profile());
    // println!("resolution = {:?}", apr.resolution());
    let prs = read_profiles(&mut dev).unwrap();
    println!("{}", serde_yaml::to_string(&prs).unwrap());
}
