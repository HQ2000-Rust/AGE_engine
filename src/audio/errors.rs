use cpal::DevicesError;
use rodio::cpal::{BackendSpecificError, DeviceNameError};
use rodio::{PlayError, StreamError, decoder::DecoderError};
use std::error::Error;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum AudioError {
    //also temp
    Default,
    IoError,
    DecoderError,
    FileNotLoaded,
    DeviceError(BackendSpecificError),
    NoDevice,
    //maybe unused
    #[allow(unused)]
    Other(Box<dyn Error + Send>),
}

//I still specify the full paths to show they're no std/core types
impl From<rodio::StreamError> for AudioError {
    fn from(value: StreamError) -> Self {
        use rodio::StreamError as SE;
        match value {
            //temporary match-all
            SE::NoDevice => AudioError::Default,
            //SE::
            _ => AudioError::Default,
        }
    }
}

impl From<rodio::DevicesError> for AudioError {
    fn from(value: rodio::DevicesError) -> Self {
        use rodio::DevicesError as SE;
        //will replace it later, is already a catch-all
        match value {
            SE::BackendSpecific { err } => AudioError::DeviceError(err),
        }
    }
}

impl From<rodio::cpal::DeviceNameError> for AudioError {
    fn from(value: DeviceNameError) -> Self {
        use DeviceNameError as DNE;
        match value {
            DNE::BackendSpecific { err } => AudioError::DeviceError(err),
        }
    }
}

impl From<std::io::Error> for AudioError {
    fn from(value: std::io::Error) -> Self {
        match value {
            //temp
            _ => AudioError::Default,
        }
    }
}

impl From<rodio::PlayError> for AudioError {
    fn from(value: PlayError) -> Self {
        match value {
            PlayError::NoDevice => AudioError::NoDevice,
            //to be specified later
            PlayError::DecoderError(_) => AudioError::DecoderError,
        }
    }
}

impl From<rodio::decoder::DecoderError> for AudioError {
    fn from(value: rodio::decoder::DecoderError) -> Self {
        match value {
            //also kinda temp match-all
            DecoderError::DecodeError(_) => AudioError::DecoderError,
            DecoderError::IoError(_) => AudioError::IoError,
            _ => AudioError::DecoderError,
        }
    }
}
