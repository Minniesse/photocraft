//! Filter › Plug-ins in a build without the `plugins` feature: no plug-in is installed and the
//! `plugin.*` commands are unknown, so there are no plug-in rows and no plug-in menu ids.

use serde_json::Value;

use crate::PhotocraftApp;
use craft::menu::Row;

/// No plug-in rows.
pub fn rows(_: &PhotocraftApp, _: &mut Vec<Row>) {}

/// Not a plug-in menu id: every id goes on to the engine, where `plugin.*` is unknown.
pub fn is_enabled(_: &PhotocraftApp, _: &str) -> Option<bool> {
    None
}

/// Not a plug-in menu id (see [`is_enabled`]).
pub fn menu(_: &mut PhotocraftApp, _: &str, _: &Value) -> Option<Result<Value, String>> {
    None
}
