use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use kira::manager::{AudioManager, AudioManagerSettings, DefaultBackend};
use once_cell::sync::OnceCell;
use crate::resource_location::ResourceLocation;

pub struct Sound {
    pub path : String,
    pub resource_location : ResourceLocation
}



pub fn get_audio_manager() -> &'static Mutex<AudioManager> {
    static INSTANCE : OnceCell<Mutex<AudioManager>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::new(AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap())
    })
}