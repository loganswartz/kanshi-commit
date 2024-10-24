use core::fmt;
use serde::Deserialize;

use crate::utils::indent;

pub struct Profile {
    pub name: String,
    pub outputs: Vec<Output>,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "profile {} {{", self.name)?;
        for output in self.outputs.iter() {
            writeln!(f, "{}", indent(&output.to_string(), 1))?;
        }
        writeln!(f)?;
        for output in self.outputs.iter() {
            writeln!(f, "    exec swaymsg output \"{}\" subpixel {}", output.display_name(), if output.subpixel_hinting != "unknown" { &output.subpixel_hinting } else { "none" })?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "output \"{}\" {{", self.display_name())?;
        writeln!(f, "    {}", if self.active { "enable" } else { "disable" })?;
        if self.active {
            writeln!(f, "    mode {}x{}@{:.3}Hz", self.current_mode.width, self.current_mode.height, self.current_mode.refresh as f64 / 1000.0)?;
            writeln!(f, "    position {},{}", self.rect.x, self.rect.y)?;
            writeln!(f, "    scale {}", self.scale)?;
            writeln!(f, "    transform {}", self.transform)?;
            writeln!(f, "    adaptive_sync {}", if self.adaptive_sync_status == "enabled" { "on" } else { "off" })?;
        }
        writeln!(f, "}}")
    }
}

impl Output {
    fn is_embedded(&self) -> bool {
        self.name.starts_with("eDP")
    }

    fn stable_name(&self) -> String {
        format!("{} {} {}", self.make, self.model, self.serial)
    }

    fn display_name(&self) -> String {
        // eDP-X denotes an embedded display, and those identifiers are typically more convenient
        // to use than the make/model/serial combination.
        if self.is_embedded() {
            self.name.clone()
        } else {
            self.stable_name()
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Output {
    pub id: usize,
    pub r#type: String,
    pub orientation: String,
    pub layout: String,
    pub rect: OutputRect,
    pub name: String,
    pub primary: bool,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub modes: Vec<OutputMode>,
    pub active: bool,
    pub scale: f32,
    pub scale_filter: String,
    pub transform: String,
    pub adaptive_sync_status: String,
    pub current_mode: OutputMode,
    pub subpixel_hinting: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct OutputRect {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct OutputMode {
    pub width: usize,
    pub height: usize,
    pub refresh: usize,
    pub picture_aspect_ratio: String,
}
