use super::audio_handles::output_markers::{OutputDisabled, OutputEnabled};
use super::device::Device;
use super::errors::AudioError;
use super::output::*;
use rodio::{OutputStream, OutputStreamHandle, Sink, source::Source, Decoder};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::marker::PhantomData;
use std::path::Path;

pub mod output_markers {
    use crate::audio::audio_handles::input_markers::InputDisabled;

    pub struct OutputDisabled;
    //pub(super) type OutDis=OutputDisabled;
    pub struct OutputEnabled;
    //pub(super) type OutEn=OutputEnabled;
}
pub mod input_markers {
    pub struct InputEnabled;
    //pub(super) type InEn=InputEnabled;
    pub struct InputDisabled;
    //pub(super) type InDis=InputDisabled;
}

//statetype pattern, everything else than the impl combinations is basically ignored, so you could have a OutputHandle<String>, but it just does...nothing
//I thought this is better for the dev using this library since the type system helps them
//I think I can't really avoid a bit of shared code if I want to keep it straightforward...
pub struct OutputHandle<O> {
    pub(super) output_handle: Option<Sink>,
    pub(super) device_name: Option<String>,
    pub(super) loaded_files: HashMap<String, Cursor<Vec<u8>>>,
    _marker: PhantomData<O>,
}

impl Default for OutputHandle<OutputDisabled> {
    fn default() -> OutputHandle<OutputDisabled> {
        OutputHandle {
            output_handle: None,
            device_name: None,
            loaded_files: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl OutputHandle<OutputDisabled> {
    //NEVER set the handle to Some(_) here!!!
    fn new() -> OutputHandle<OutputDisabled> {
        Default::default()
    }
    fn load_file(&mut self, path: &str, new_name: String) -> Result<(), AudioError> {
        //capacity: 50 Kilobytes, every sound effect has at least this size...
        let mut buffer = Vec::with_capacity(1024 * 50);
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
        let buffer = Cursor::new(buffer);
        self.loaded_files.insert(new_name, buffer);
        Ok(())
    }
    fn unload_file(&mut self, name: String) -> Result<(), AudioError> {
        if self.loaded_files.remove(&name).is_none() {
            return Err(AudioError::FileNotLoaded);
        }
        Ok(())
    }

    //maybe one day I'll find a shorter name...
    pub fn activate_output_with_default_device(self) -> Result<OutputHandle<OutputEnabled>, AudioError> {
        //IDK what this was supposed to do yesterday, but I'll leave it here
        if self.output_handle.is_none() {}
        //--
        let (_stream, stream_handle) = OutputStream::try_default()?;
        OutputHandle::from(self, stream_handle)

    }

    fn activate_output_with_device(self, device: Device<Output>) -> Result<OutputHandle<OutputEnabled>, AudioError> {
        let (_stream, stream_handle) = OutputStream::try_from_device(&device.device)?;
        //self.output_handle = Some(Sink::try_new(&stream_handle)?);
        Ok(OutputHandle::from(self, stream_handle)?)
    }
}



//important!
//NO get_device (but get_device_name) because it instanly gives an error if you try to use a device that just can't work for whatever reason
//meaning you directly get an error
impl OutputHandle<OutputEnabled> {

    fn from(handler: OutputHandle<OutputDisabled>, stream_handle: OutputStreamHandle) -> Result<Self, AudioError> {
        Ok(
        Self {
            output_handle: Some(Sink::try_new(&stream_handle)?),
            device_name: handler.device_name,
            loaded_files: handler.loaded_files,
            _marker: PhantomData,
        }
        )
    }

    pub fn load_file(&mut self, path: &Path, new_name: String) -> Result<(), AudioError> {
        //capacity: 50 Kilobytes, every sound effect has at least this size...
        let mut buffer = Vec::with_capacity(1024 * 50);
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
        let buffer = Cursor::new(buffer);
        self.loaded_files.insert(new_name, buffer);
        Ok(())
    }
    fn unload_file(&mut self, name: String) -> Result<(), AudioError> {
        if self.loaded_files.remove(&name).is_none() {
            Err(AudioError::FileNotLoaded)
        } else {
            Ok(())
        }
    }

    /*fn get_available_devices() -> Result<Vec<Device<Output>>, AudioError> {
        rodio::Device::
    }*/

    fn use_default_device(&mut self) -> Result<(), AudioError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        self.output_handle=Some(Sink::try_new(&stream_handle)?);
        Ok(())
    }
    fn change_device(&mut self, device: Device<Output>) {
        todo!();
    }

    fn get_device_name(&self) -> String {
        //ask the compiler why this works...
        <Option<String> as Clone>::clone(&self.device_name)
            .expect("Impossible error. Device<OutputEnabled> always has device_name ≠ None")
    }

    fn disable_output(&mut self) {
        self.output_handle = None;
    }

    fn play_from_file(&mut self /*...*/, new_id: u32) {
        todo!();
    }
    pub fn play_loaded(&mut self, name: String, new_id: u32) -> Result<(), AudioError> {
        if dbg!(self.loaded_files.contains_key(&name)) {
            let buffer=self.loaded_files.get(&name)
                .expect("Impossible, just checked if it's there")
                .to_owned();
            self.output_handle.as_mut().expect("Impossible, thanks to the type system")
                .append(Decoder::new(buffer)?);
        }
        Ok(())
    }

    //Result!!
    fn pause(&mut self, id: u32) {

    }

    fn pause_all(&mut self) {

    }

    fn is_paused(&self) -> bool {
        self.output_handle.as_ref().expect("Impossible...").is_paused()
    }
    //other play functions...
    //channels?
    //ids?
}
