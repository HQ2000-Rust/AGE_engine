use super::device::Device;
use crate::audio::errors::AudioError;
use rodio::DeviceTrait;

pub struct Output;
impl Device<Output> {
    fn get_name(&self) -> Result<String, AudioError> {
        //needed because otherwise I'd also need to implement From<Result<String, DeviceNameError>> for Result<String, AudioError>...
        Ok(self.device.name()?)
    }
}
//re-exports
