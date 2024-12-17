use hidraw::{Device, Result};

use crate::profile::{NUM_PROFILES, ActiveProfile, Profile, Profiles};

const ACTIVE_PROFILE_REPORT_ID: u8 = 0xF0;
const PROFILE_REPORT_ID: [u8; NUM_PROFILES] = [0xF3, 0xF4, 0xF5];

pub fn get_active_profile(dev: &mut Device) -> Result<ActiveProfile> {
    dev.get_feature_report::<ActiveProfile>(ACTIVE_PROFILE_REPORT_ID)
}

pub fn set_active_profile(dev: &mut Device, profile: u8) -> Result<()> {
    let ap = ActiveProfile::profile_request(ACTIVE_PROFILE_REPORT_ID, profile);
    dev.send_feature_report::<ActiveProfile>(&ap)
}

pub fn set_resolution(dev: &mut Device, resolution: u8) -> Result<()> {
    let ap = ActiveProfile::resolution_request(ACTIVE_PROFILE_REPORT_ID, resolution);
    dev.send_feature_report::<ActiveProfile>(&ap)
}

fn read_profile(dev: &mut Device, id: u8) -> Result<Profile> {
    dev.get_feature_report::<Profile>(id)
}

pub fn read_profiles(dev: &mut Device) -> Result<Profiles> {
    let mut profiles = Vec::new();
    for i in 0..NUM_PROFILES {
        profiles.push(read_profile(dev, PROFILE_REPORT_ID[i])?)
    }
    Ok(Profiles {
        profiles: profiles.try_into().unwrap(),
    })
}

fn write_profile(dev: &mut Device, id: u8, profile: &mut Profile) -> Result<()> {
    profile.id = id;
    dev.send_feature_report::<Profile>(profile)
}

pub fn write_profiles(dev: &mut Device, profiles: &mut Profiles) -> Result<()> {
    for i in 0..NUM_PROFILES {
        write_profile(dev, PROFILE_REPORT_ID[i], &mut profiles.profiles[i])?
    }
    Ok(())
}
