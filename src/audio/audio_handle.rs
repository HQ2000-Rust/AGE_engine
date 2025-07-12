use super::errors::AudioError;
use super::output::*;
use super::output_handle::output_markers::{OutputDisabled, OutputEnabled};
use super::traits::marker::OutputHandlerState;
use cpal::Stream;
use cpal::traits::{DeviceTrait, HostTrait};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, source::Source};
use std::any::type_name_of_val;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::marker::PhantomData;
use std::path::Path;

pub mod output_markers {
    use crate::audio::output_handle::input_markers::InputDisabled;

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

pub struct OutputHandle<O> {
    pub(super) stream: Option<OutputStream>,
    pub(super) stream_handle: Option<OutputStreamHandle>,
    pub(super) sink: Option<Sink>,
    pub(super) loaded_files: HashMap<String, Cursor<Vec<u8>>>,
    _marker: PhantomData<O>,
}

impl Default for OutputHandle<OutputDisabled> {
    fn default() -> OutputHandle<OutputDisabled> {
        OutputHandle {
            stream: None,
            stream_handle: None,
            sink: None,
            loaded_files: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<O: OutputHandlerState> std::fmt::Debug for OutputHandle<O> {
    //tmp
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\n{}:\nHandle_is_none: {}\n- loaded files: {:?}\n",
            {
                match type_name_of_val(self) {
                    "age_engine::audio::audio_handles::OutputHandle<age_engine::audio::audio_handles::output_markers::OutputEnabled>" => {
                        "OutputHandle<OutputEnabled>"
                    }
                    "age_engine::audio::audio_handles::OutputHandle<age_engine::audio::audio_handles::output_markers::OutputDisabled>" => {
                        "OutputHandle<OutputDisabled>"
                    }
                    _ => unreachable!(),
                }
            },
            self.stream_handle.is_none(),
            self.loaded_files.keys().collect::<Vec<&String>>(),
        )
    }
}

impl<O: OutputHandlerState> OutputHandle<O> {
    //I'm not that good of a Rust dev, I saw this type trick in the stdlib
    pub fn load_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        new_name: String,
    ) -> Result<(), AudioError> {
        //capacity: 50 Kilobytes, every sound effect has at least this size...
        let mut buffer = Vec::with_capacity(1024 * 50);
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
        let data = Cursor::new(buffer);
        self.loaded_files.insert(new_name, data);
        Ok(())
    }

    //important: NON-fatal error, can be tried anytime, Result can also be safely ignored - maybe another type??
    fn unload_file(&mut self, name: String) -> Result<(), AudioError> {
        if self.loaded_files.remove(&name).is_none() {
            return Err(AudioError::FileNotLoaded);
        }
        Ok(())
    }
    pub fn get_all_loaded_files(&self) -> Vec<&String> {
        self.loaded_files.keys().collect::<Vec<&String>>()
    }
}

impl OutputHandle<OutputDisabled> {
    //NEVER set the handle to Some(_) here!!!

    pub fn new() -> OutputHandle<OutputDisabled> {
        Default::default()
    }

    //maybe one day I'll find a shorter name...
    pub fn activate_output(self) -> Result<OutputHandle<OutputEnabled>, AudioError> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let mut result = OutputHandle::<OutputEnabled> {
            stream: Some(stream),
            stream_handle: Some(stream_handle),
            sink: None,
            loaded_files: self.loaded_files,
            _marker: PhantomData,
        };
        result.sink = match result.stream_handle {
            //match because otherwise I can't destructure the Option without taking ownership of the stream_handle
            Some(ref stream_handle) => {
                Some(Sink::try_new(&stream_handle)?)
            },
            None => {
                unreachable!();
            }
        };
        Ok(result)
    }
}

//important!
//NO get_device (but get_device_name) because it instantly gives an error if you try to use a device that just can't work for whatever reason
//meaning you directly get an error
impl OutputHandle<OutputEnabled> {

    fn disable_output(self) -> OutputHandle<OutputDisabled> {
        OutputHandle::<OutputDisabled> {
            stream: None,
            stream_handle: None,
            sink: None,
            loaded_files: self.loaded_files,
            _marker: PhantomData,
        }
    }

    pub fn play_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AudioError> {
        let reader = BufReader::new(File::open(path)?);
        let decoder = Decoder::new(reader)?;
        self.sink.as_mut().expect("Not 'None' if the output is activated").append(decoder);
        Ok(())
    }

    pub fn play_loaded(&mut self, name: String) -> Result<(), AudioError> {
        let data=match self.loaded_files.get(&name) {
            Some(data) => data.clone(),
            None => return Err(AudioError::FileNotLoaded),
        };
        let decoder = Decoder::new(data)?;
        self.sink.as_mut().expect("Basically never 'None' if output is enabled").append(decoder);
        //dbg artifacts
        /*handle.append(decoder);
        handle.set_volume(10.0);
        dbg!(handle.len());
        dbg!(handle.volume());
        dbg!(handle.speed());
        dbg!(handle.is_paused());
        handle.sleep_until_end();
        dbg!("end");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            dbg!(handle.get_pos());
        }*/
        Ok(())
    }

    fn pause(&mut self /*, id: u32*/) {
        self.sink.as_mut().expect("Basically never 'None' if output is enabled").pause();
    }

    fn play(&mut self) {
        self.sink.as_mut().expect("Basically never 'None' if output is enabled").play();
    }

    fn is_paused(&self) -> bool {
        //again, match to destructure without taking ownership
        match self.sink {
            Some(ref sink) => sink.is_paused(),
            None => unreachable!("Basically never 'None' if output is enabled"),
        }
    }

    //other play functions...
    //channels?
    //ids?
    //(no)
}
