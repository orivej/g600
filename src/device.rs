use std::fs::{read_dir, read_to_string};
use std::io;
use std::path::{Path, PathBuf};

use hidraw::{Device, Result};

use crate::profile::{ActiveProfile, Profile, Profiles, PROFILE_REPORT_ID};
use crate::profilesio::ProfilesIO;

const ACTIVE_PROFILE_REPORT_ID: u8 = 0xF0;

pub struct G600 {
    dev: Device,
}

impl G600 {
    pub fn open(path: Option<impl AsRef<Path>>) -> Result<G600> {
        let path = match path {
            Some(path) => path.as_ref().to_path_buf(),
            None => G600::detect()?,
        };
        let pathstr = path.to_string_lossy().to_string();
        match Device::open(path) {
            Ok(dev) => Ok(G600{dev}),
            Err(err) => Err(io::Error::new(err.kind(), format!("Failed to open {}: {}", &pathstr, err))),
        }
    }

    fn detect() -> Result<PathBuf> {
        for entry in read_dir("/sys/bus/hid/devices")? {
            let entry = entry?;
            for input in read_dir(entry.path().join("input"))? {
                let input = input?;
                let name = read_to_string(input.path().join("name"))?;
                if name == "Logitech Gaming Mouse G600 Keyboard\n" {
                    if let Some(hidraw) = read_dir(entry.path().join("hidraw"))?.next() {
                        return Ok(PathBuf::from("/dev").join(hidraw?.file_name()))
                    }
                }
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "No G600 mouse found"))
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
        for id in PROFILE_REPORT_ID {
            profiles.push(self.read_profile(id)?);
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
