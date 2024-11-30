mod player;
mod audio_handling;
mod utils;
mod components;

use godot::prelude::*;

struct NativeRustAPI;

#[gdextension]
unsafe impl ExtensionLibrary for NativeRustAPI {}