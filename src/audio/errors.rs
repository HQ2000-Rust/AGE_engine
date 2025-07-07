use rodio::StreamError;
use std::io::ErrorKind;

pub(super) enum AudioError {
    AudioError,
    FileError,
    DecoderError,
    FileNotLoaded,
}

impl From<rodio::StreamError> for AudioError {
    fn from(value: StreamError) -> Self {
        use rodio::StreamError as SE;
        match value {
            //temporary match-all
            SE::NoDevice => AudioError::AudioError,
            //SE::
            _ => AudioError::AudioError,
        }
    }
}

impl From<std::io::Error> for AudioError {
    fn from(value: std::io::Error) -> Self {
        use std::io::Error as IOError;
        match value {
            //temp
            _ => AudioError::FileError,
        }
    }
}
