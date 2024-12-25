use std::convert::AsRef;
use std::fs::{write, File};
use std::io::Read;
use std::mem::transmute_copy;
use std::path::{Path, PathBuf};

use hidraw::Result;

use crate::profile::{Profiles, NUM_PROFILES, PROFILE_SIZE};

pub trait ProfilesIO {
    fn read_profiles(&mut self) -> Result<Profiles>;
    fn write_profiles(&mut self, profiles: &Profiles) -> Result<()>;
}

pub struct ProfilesDump {
    path: PathBuf,
}

impl ProfilesDump {
    pub fn new(path: impl AsRef<Path>) -> ProfilesDump {
        ProfilesDump {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl ProfilesIO for ProfilesDump {
    fn read_profiles(&mut self) -> Result<Profiles> {
        let mut buf = [0u8; NUM_PROFILES * PROFILE_SIZE];
        File::open(&self.path)?.read_exact(&mut buf)?;
        Ok(unsafe { transmute_copy(&buf) })
    }

    fn write_profiles(&mut self, profiles: &Profiles) -> Result<()> {
        let buf: [u8; NUM_PROFILES * PROFILE_SIZE] = unsafe { transmute_copy(profiles) };
        write(&self.path, buf)
    }
}
