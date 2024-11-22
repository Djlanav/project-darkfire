mod audio_handling;
mod utils;

use godot::prelude::*;

struct NativeRustAPI;

#[gdextension]
unsafe impl ExtensionLibrary for NativeRustAPI {}