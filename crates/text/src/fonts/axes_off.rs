//! The axes and named instances of a variable face, read with skrifa, for a build without the
//! `font-download` feature: craft's font manager (which carries these readers) also brings the font
//! store's downloader, so this build reads them here. Same types and results as
//! `craft::font_manager::axes`.

use skrifa::string::StringId;
use skrifa::{FontRef, MetadataProvider as _};

/// One variation axis of a face.
#[derive(Clone, Debug, PartialEq)]
pub struct FontAxis {
    /// The OpenType tag (`wght`, `wdth`, `slnt`, `ital`, `opsz`, or a custom one).
    pub tag: [u8; 4],
    /// The axis's name from the font's name table (`Weight`), English when there is one.
    pub name: String,
    /// Lowest value, in the axis's own units.
    pub min: f32,
    /// Value the face uses when no coordinate is set.
    pub default: f32,
    /// Highest value, in the axis's own units.
    pub max: f32,
    /// Hidden from the user by the font (`HIDDEN_AXIS`).
    pub hidden: bool,
}

/// A named instance of a variable face (`Bold Condensed`): its coordinates per axis.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedInstance {
    /// The instance's name from the font's name table.
    pub name: String,
    /// (tag, value) for every axis of the face, in axis order.
    pub coordinates: Vec<([u8; 4], f32)>,
}

/// The axes of the face at `index` in `data`, in font order; empty for a static font.
pub fn read_axes(data: &[u8], index: u32) -> Vec<FontAxis> {
    let Ok(font) = FontRef::from_index(data, index) else { return Vec::new() };
    font.axes()
        .iter()
        .map(|axis| FontAxis {
            tag: axis.tag().to_be_bytes(),
            name: name(&font, axis.name_id()).unwrap_or_default(),
            min: axis.min_value(),
            default: axis.default_value(),
            max: axis.max_value(),
            hidden: axis.is_hidden(),
        })
        .collect()
}

/// The named instances of the face at `index` in `data`, in font order.
pub fn read_named_instances(data: &[u8], index: u32) -> Vec<NamedInstance> {
    let Ok(font) = FontRef::from_index(data, index) else { return Vec::new() };
    let tags: Vec<[u8; 4]> = font.axes().iter().map(|axis| axis.tag().to_be_bytes()).collect();
    font.named_instances()
        .iter()
        .map(|instance| NamedInstance {
            name: name(&font, instance.subfamily_name_id()).unwrap_or_default(),
            coordinates: tags.iter().copied().zip(instance.user_coords()).collect(),
        })
        .collect()
}

fn name(font: &FontRef, id: StringId) -> Option<String> {
    font.localized_strings(id).english_or_first().map(|s| s.to_string()).filter(|s| !s.is_empty())
}
