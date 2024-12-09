mod player;
mod audio_handling;
mod utils;
mod components;
mod input_processing;

use godot::prelude::*;

struct NativeRustAPI;

#[gdextension]
unsafe impl ExtensionLibrary for NativeRustAPI {}