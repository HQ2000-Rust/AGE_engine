#![cfg(test)]

use age_engine::audio::output_handle::OutputHandle;
use std::path::Path;
#[test]
fn play_test() {
    let handle = OutputHandle::default();
    let mut handle = handle.activate_output().unwrap();
    //handle.load_file("music.mp3", String::from("1")).unwrap();
    //handle.play_loaded(String::from("1"),1).unwrap();
    handle.play_from_file("music.mp3").unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
