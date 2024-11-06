use std::{io::Read, process::Command};

/// Load the configuration from the sway IPC.
pub fn load_configuration_from_ipc() -> Result<String, Box<dyn std::error::Error>> {
    let Ok(output) = Command::new("swaymsg").args(["-t", "get_outputs", "--raw"]).output() else {
        eprintln!("Failed to run 'swaymsg -t get_outputs', do you have sway installed?");
        std::process::exit(1);
    };

    Ok(String::from_utf8(output.stdout)?)
}

/// Get the configuration from the given source.
///
/// If the source is None, the configuration is read from the sway IPC.
pub fn get_configuration(source: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let Some(filename) = source else {
        return load_configuration_from_ipc();
    };

    let mut result = String::new();

    if filename == "-" {
        std::io::stdin().read_to_string(&mut result)?;
    } else {
        std::fs::File::open(filename)?.read_to_string(&mut result)?;
    };

    Ok(result)
}
