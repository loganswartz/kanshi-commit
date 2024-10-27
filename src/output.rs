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
            let (key, value) = output.subpixel_hinting();
            writeln!(f, r#"    exec swaymsg output "'{}'" {} {}"#, output.display_name(), key, value)?;
        }

        write!(f, "}}")
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"output "{}" {}"#, self.display_name(), if self.active { "enable" } else { "disable" })?;

        if self.active {
            let params = vec![
                self.mode(),
                self.position(),
                self.scale(),
                self.transform(),
                self.adaptive_sync(),
            ];

            for (key, value) in params.iter() {
                write!(f, " {} {}", key, value)?;
            }
        }

        writeln!(f)
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

    fn mode(&self) -> (String, String) {
        (String::from("mode"), format!("{}x{}@{:.3}Hz", self.current_mode.width, self.current_mode.height, self.current_mode.refresh as f64 / 1000.0))
    }

    fn position(&self) -> (String, String) {
        (String::from("position"), format!("{},{}", self.rect.x, self.rect.y))
    }

    fn scale(&self) -> (String, String) {
        (String::from("scale"), self.scale.to_string())
    }

    fn transform(&self) -> (String, String) {
        (String::from("transform"), self.transform.clone())
    }

    fn adaptive_sync(&self) -> (String, String) {
        (String::from("adaptive_sync"), String::from(if self.adaptive_sync_status == "enabled" { "on" } else { "off" }))
    }

    fn subpixel_hinting(&self) -> (String, String) {
        (String::from("subpixel"), if self.subpixel_hinting != "unknown" { self.subpixel_hinting.clone() } else { String::from("none") })
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
