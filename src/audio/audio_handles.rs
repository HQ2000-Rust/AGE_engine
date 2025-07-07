use super::errors::AudioError;
use rodio::{OutputStream, OutputStreamHandle, source::Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::marker::PhantomData;
use crate::audio::audio_handles::input_markers::{InputDisabled, InputEnabled};
use crate::audio::audio_handles::output_markers::{OutputDisabled, OutputEnabled};


pub(super) mod output_markers {
    use crate::audio::audio_handles::input_markers::InputDisabled;

    pub(super) struct OutputDisabled;
    //pub(super) type OutDis=OutputDisabled;
    pub(super) struct OutputEnabled;
    //pub(super) type OutEn=OutputEnabled;
}
pub(super) mod input_markers {
    pub(super) struct InputEnabled;
    //pub(super) type InEn=InputEnabled;
    pub(super) struct InputDisabled;
    //pub(super) type InDis=InputDisabled;
}

//statetype pattern, everything else than the impl combinations is basically ignored, so you could have a OutputHandle<String>, but it just does...nothing
//I thought this is better for the dev using this library since the type system helps them
//I think I can't really avoid a bit of shared code if I want to keep it straightforward...
pub(super) struct OutputHandler<O>
{
    pub(super) output_handle: Option<OutputStreamHandle>,
    pub(super) loaded_files: HashMap<String, Cursor<Vec<u8>>>,
    _marker: PhantomData<O>,
}

impl Default for OutputHandler<OutputDisabled> {
    fn default() -> OutputHandler<OutputDisabled> {
        OutputHandler {
            output_handle: None,
            loaded_files: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl OutputHandler<OutputDisabled> {
    fn load_file(&mut self, path: &str, new_name: String) -> Result<(), AudioError> {
        //capacity: 50 Kilobytes, every sound effect has at least this size...
        let mut buffer = Vec::with_capacity(1024*50);
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
    fn activate_output_default(&mut self) -> Result<(),AudioError>{
        if self.output_handle.is_none() {
            let (_stream, stream_handle) = OutputStream::?;
            self.output_handle = Some(stream_handle);
        }
        Ok(())
    }
    fn activate_output_from_device() {
        todo!();
    }
}

impl OutputHandler<OutputEnabled> {
    fn new() -> OutputHandler<OutputDisabled> {
        Default::default()
    }
    fn load_file(&mut self, path: &str, new_name: String) -> Result<(), AudioError> {
        //capacity: 50 Kilobytes, every sound effect has at least this size...
        let mut buffer = Vec::with_capacity(1024*50);
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
    
    fn disable_output(&mut self) {
        self.output_handle = None;
    }    
    fn play(&mut self) {
        todo!();
    }
    //other play functions...
    //channels?
    //ids?
}