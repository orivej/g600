use std::fmt;
use serde::{Deserialize, Serialize};
use static_assertions::const_assert_eq;

// Buttons (as labeled on the mouse):
// 1 2 6: index, middle, ring finger buttons
// 3 4 5: wheel down, left, right
// 8 7: below wheel
// 9-20: thumb buttons
const NUM_BUTTONS: usize = 20;
const NUM_DPI: usize = 4;
pub const NUM_PROFILES: usize = 3;
// const DPI_MIN: u16 = 200;
// const DPI_MAX: u16 = 8200;

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
pub struct Profile {
    #[serde(skip_serializing, default)]
    pub id: u8,
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
pub struct Profiles {
    pub profiles: [Profile; NUM_PROFILES],
}

impl Profiles {
    pub fn propagate_gshift(&mut self) {
        for profile in self.profiles.iter_mut() {
            profile.propagate_gshift();
        }
    }
}

#[repr(C, packed)]
pub struct ActiveProfile {
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
