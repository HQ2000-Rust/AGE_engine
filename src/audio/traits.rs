use super::audio_handles::{input_markers, output_markers};

//maybe I will remove them idk
pub mod marker {
    use crate::audio::audio_handles::{input_markers, output_markers};

    pub trait OutputHandlerState {}
    impl OutputHandlerState for output_markers::OutputDisabled {}
    impl OutputHandlerState for output_markers::OutputEnabled {}
    pub trait InputHandlerState {}
    impl InputHandlerState for input_markers::InputDisabled {}
    impl InputHandlerState for input_markers::InputEnabled {}
}
