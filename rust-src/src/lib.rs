mod audio_handling;

use godot::prelude::*;

struct NativeRustAPI;

#[gdextension]
unsafe impl ExtensionLibrary for NativeRustAPI {}