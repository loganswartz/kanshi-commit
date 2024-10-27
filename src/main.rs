use std::{io::Read, path::PathBuf, process::Command};

use clap::Parser;

mod output;
mod utils;

use output::{Output, Profile};

fn load_configuration_from_ipc() -> Result<String, Box<dyn std::error::Error>> {
    let Ok(output) = Command::new("swaymsg").args(["-t", "get_outputs", "--raw"]).output() else {
        eprintln!("Failed to run 'swaymsg -t get_outputs', do you have sway installed?");
        std::process::exit(1);
    };

    Ok(String::from_utf8(output.stdout)?)
}

/// Get the configuration from the given source.
///
/// If the source is None, the configuration is read from the sway IPC.
fn get_configuration(source: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
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

#[derive(Parser)]
struct Args {
    /// The name of the new profile.
    profile: String,

    /// Save the profile to the configuration directory.
    #[clap(short, long)]
    save: bool,

    /// When saving, allow overwriting an existing profile of the same name.
    #[clap(short, long)]
    force: bool,

    /// The suffix added to the profile filename when saving.
    #[clap(long, default_value = ".conf")]
    suffix: String,

    /// The directory where the profile should be saved [default: $XDG_CONFIG_HOME/kanshi/config.d].
    #[clap(short, long)]
    config_dir: Option<PathBuf>,

    /// The input file to read the current display configuration from.
    #[clap(long)]
    from_file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.force && !args.save {
        eprintln!("The --force flag cannot be used without --save.");
        std::process::exit(1);
    } else if args.config_dir.is_some() && !args.save {
        eprintln!("The --config-dir flag cannot be used without --save.");
        std::process::exit(1);
    }

    let configuration = get_configuration(args.from_file.as_deref())?;
    let Ok(outputs) = serde_json::from_str::<Vec<Output>>(&configuration) else {
        eprintln!("Failed to parse the configuration.");
        std::process::exit(1);
    };

    let profile = Profile {
        name: args.profile,
        outputs,
    };

    if args.save {
        let default_config_dir = dirs::config_dir().map(|v| v.join("kanshi/config.d"));
        let config_dir = args.config_dir.or(default_config_dir).expect("could not find a valid configuration directory");
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        let filename = format!("{}{}", &profile.name, args.suffix);
        let profile_path = config_dir.join(&filename);
        if profile_path.exists() && !args.force {
            eprintln!("Profile {} already exists. Use --force to overwrite.", &filename);
            std::process::exit(1);
        }

        std::fs::write(&profile_path, profile.to_string())?;

        println!("Profile saved to {}", profile_path.display());
    } else {
        println!("{}", profile);
    }

    Ok(())
}
