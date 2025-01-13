use std::fmt;

use enumset::{EnumSet, EnumSetType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use static_assertions::const_assert_eq;

use crate::hidkey::HidKey;

// Buttons (as labeled on the mouse):
// 1 2 6: index, middle, ring finger buttons
// 3 4 5: wheel down, left, right
// 8 7: below wheel
// 9-20: thumb buttons
const NUM_BUTTONS: usize = 20;
const NUM_DPI: usize = 4;
pub const NUM_PROFILES: usize = 3;
pub const PROFILE_SIZE: usize = 154;
// const DPI_MIN: u16 = 200;
// const DPI_MAX: u16 = 8200;
pub const PROFILE_REPORT_ID: [u8; NUM_PROFILES] = [0xF3, 0xF4, 0xF5];

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
    DPIUp,        // dpis: 0 -> 1 -> 2 -> 3 -> 3
    DPIDown,      // dpis: 3 -> 2 -> 1 -> 0 -> 0
    DPICycle,     // dpis: 0 -> 1 -> 2 -> 3 -> 0
    ProfileCycle, // Default for G8 in all profiles
    DPIShift,     // dpi = dpi_shift while DPIShift button is pressed.
    DPIDefault,   // dpis: -> dpi_default
    GShift,       // Default for ring finger (G6) in the first two profiles
    M11a,         // Same effect as M11
    M12a,         // Same effect as M12
    X1A,
    X1B,
    X1C,
}

impl ButtonAction {
    fn is_default(&self) -> bool {
        matches!(*self, ButtonAction::Key)
    }
}

#[derive(Debug, Serialize, Deserialize, EnumSetType)]
#[enumset(repr = "u8", serialize_repr = "list")]
enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Meta,
    RCtrl,
    RShift,
    RAlt,
    RMeta,
}

type Modifiers = EnumSet<Modifier>;

type Color = [u8; 3];

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Button {
    #[serde(skip_serializing_if = "ButtonAction::is_default", default)]
    action: ButtonAction,
    #[serde(skip_serializing_if = "EnumSet::is_empty", default)]
    modifiers: Modifiers,
    #[serde(skip_serializing_if = "HidKey::is_default", default)]
    key: HidKey,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct Dpi(u8);

impl Serialize for Dpi {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_u16(u16::from(self.0) * 50)
    }
}

impl<'de> Deserialize<'de> for Dpi {
    fn deserialize<D>(d: D) -> Result<Dpi, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dpi = u16::deserialize(d)? / 50;
        Ok(Dpi(u8::try_from(dpi).map_err(serde::de::Error::custom)?))
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(skip_serializing, default)]
    id: u8,
    #[serde(
        serialize_with = "hex::serialize_upper",
        deserialize_with = "hex::deserialize"
    )]
    led_color: Color,
    led_effect: LedEffect,
    led_duration: u8, // 0 to 15 seconds
    #[serde(skip_serializing, default)]
    unknown1: [u8; 5],
    report_rate: ReportRate,
    dpi_shift: Dpi,       // dpi is between 200 and 8200; 0 is disabled
    dpi_default: u8,      // between 1 and 4
    dpis: [Dpi; NUM_DPI], // dpi is between 200 and 8200; 0 is disabled
    #[serde(skip_serializing, default)]
    unknown2: [u8; 13],
    buttons: [Button; NUM_BUTTONS],
    #[serde(
        serialize_with = "hex::serialize_upper",
        deserialize_with = "hex::deserialize"
    )]
    g_led_color: Color,
    g_buttons: [Button; NUM_BUTTONS],
}

const_assert_eq!(std::mem::size_of::<Profile>(), PROFILE_SIZE);

impl Profile {
    fn propagate_gshift(&mut self) {
        for (button, g_button) in self.buttons.iter().zip(self.g_buttons.iter_mut()) {
            if matches!(button.action, ButtonAction::GShift) {
                g_button.action = ButtonAction::GShift;
            }
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles {
    pub profiles: [Profile; NUM_PROFILES],
}

const_assert_eq!(std::mem::size_of::<Profiles>(), NUM_PROFILES * PROFILE_SIZE);

impl Profiles {
    pub fn fix_ids(&mut self) {
        for (profile, id) in self.profiles.iter_mut().zip(PROFILE_REPORT_ID) {
            profile.id = id;
        }
    }

    pub fn propagate_gshift(&mut self) {
        for profile in &mut self.profiles {
            profile.propagate_gshift();
        }
    }
}

#[repr(C, packed)]
pub struct ActiveProfile {
    id: u8,
    info: u8, // unused:1, resolution:2, unused:1, profile:2, set_resolution:1, set_profile: 1
    unused: u16,
}

impl ActiveProfile {
    fn profile(&self) -> u8 {
        (self.info >> 4) & 3
    }

    fn resolution(&self) -> u8 {
        (self.info >> 1) & 3
    }

    pub fn profile_request(id: u8, profile: u8) -> ActiveProfile {
        ActiveProfile {
            id,
            info: 0x80 | (profile << 4),
            unused: 0,
        }
    }

    pub fn resolution_request(id: u8, resolution: u8) -> ActiveProfile {
        ActiveProfile {
            id,
            info: 0x40 | (resolution << 1),
            unused: 0,
        }
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
