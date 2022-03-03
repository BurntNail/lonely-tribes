use crate::config::LTConfig;
use amethyst::{
    assets::Loader,
    audio::{AudioSink, Mp3Format, SourceHandle},
    ecs::{World, WorldExt},
};
use std::{iter::Cycle, vec::IntoIter};

const BACKGROUND_TRACKS: &[&str] = &[
    "music/DOS-88/Checking Manifest.mp3",
    "music/DOS-88/DOS-88 - Far Away.mp3",
    "music/DOS-88/Parabola.mp3",
    "music/DOS-88/Race to Mars.mp3",
    "music/DOS-88/Smooth Sailing.mp3",
];

pub struct Muzac {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

pub fn init_audio(world: &mut World) {
    let vol = LTConfig::new().conf.vol;

    if vol <= 0.0 {
        return;
    }

    let music = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25 * vol);

        let music = BACKGROUND_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        Muzac { music }
    };

    world.insert(music);
}

#[inline(always)]
pub fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, Mp3Format, (), &world.read_resource())
}
