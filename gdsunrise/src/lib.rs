use godot::prelude::*;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

mod warp_server;
mod kuzu_server;
mod twitch_server;
