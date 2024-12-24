use hidraw::{Device, Result};

use crate::profile::{ActiveProfile, Profile, Profiles, NUM_PROFILES, PROFILE_REPORT_ID};
use crate::profilesio::ProfilesIO;

const ACTIVE_PROFILE_REPORT_ID: u8 = 0xF0;

pub struct G600 {
    dev: Device,
}

impl G600 {
    pub fn new(dev: Device) -> G600 {
        G600 { dev }
    }

    pub fn open(path: String) -> Result<G600> {
        Ok(G600::new(Device::open(path)?))
    }

    pub fn get_active_profile(&mut self) -> Result<ActiveProfile> {
        self.dev.get_feature_report::<ActiveProfile>(ACTIVE_PROFILE_REPORT_ID)
    }

    pub fn set_active_profile(&mut self, profile: u8) -> Result<()> {
        let ap = ActiveProfile::profile_request(ACTIVE_PROFILE_REPORT_ID, profile);
        self.dev.send_feature_report::<ActiveProfile>(&ap)
    }

    pub fn set_resolution(&mut self, resolution: u8) -> Result<()> {
        let ap = ActiveProfile::resolution_request(ACTIVE_PROFILE_REPORT_ID, resolution);
        self.dev.send_feature_report::<ActiveProfile>(&ap)
    }

    fn read_profile(&mut self, id: u8) -> Result<Profile> {
        self.dev.get_feature_report::<Profile>(id)
    }

    fn write_profile(&mut self, profile: &Profile) -> Result<()> {
        self.dev.send_feature_report::<Profile>(profile)
    }

}

impl ProfilesIO for G600 {
    fn read_profiles(&mut self) -> Result<Profiles> {
        let mut profiles = Vec::new();
        for i in 0..NUM_PROFILES {
            profiles.push(self.read_profile(PROFILE_REPORT_ID[i])?);
        }
        Ok(Profiles {
            profiles: profiles.try_into().unwrap(),
        })
    }

    fn write_profiles(&mut self, profiles: &Profiles) -> Result<()> {
        for profile in profiles.profiles.iter() {
            self.write_profile(profile)?;
        }
        Ok(())
    }
}
