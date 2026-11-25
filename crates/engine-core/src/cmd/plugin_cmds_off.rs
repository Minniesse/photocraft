//! The `plugin.*` commands in a build without the `plugins` feature: no WebAssembly plug-in host.
//! No `plugin.*` command is registered, so each is an unknown command; a smart filter that names a
//! plug-in (`plugin.run`) is skipped, as when its plug-in isn't installed; the Plug-ins preference
//! folder is never read.

use photocraft_raster::Surface;
use serde_json::Value;

use crate::Session;
use crate::commands::CommandSpec;

/// The id a plug-in smart filter is recorded under.
pub const RUN: &str = "plugin.run";

/// No `plugin.*` commands.
pub fn specs() -> Vec<CommandSpec> {
    Vec::new()
}

/// A recorded plug-in smart filter draws nothing: there is no host to run it.
pub(crate) fn apply_to_surface(_: &str, _: &Value, _: &Surface, _: Option<&Surface>, _: photocraft_geom::Rect) -> Option<Surface> {
    None
}

/// `k`: no plug-in is installed.
pub fn preview_factor(_: &Value, k: u32) -> u32 {
    k
}

/// Nothing changes: no plug-in is installed.
pub fn scale_for_preview(_: &mut Value, _: u32) {}

/// The Plug-ins preference folder is not read.
pub(crate) fn sync_prefs(_: &Session) {}
