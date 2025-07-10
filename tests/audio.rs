#![cfg(test)]

use std::path::Path;
use age_engine::audio::audio_handles::OutputHandle;
#[test]
fn play_test() {
    let handle=OutputHandle::default();
    let mut handle=handle.activate_output_with_default_device().unwrap();
    handle.load_file(Path::new("music.mp3"), String::from("1")).unwrap();
    handle.play_loaded(String::from("1"),1).unwrap();
}
