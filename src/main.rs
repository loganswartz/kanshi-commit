use std::{path::PathBuf, process::Command};

use clap::Parser;

mod output;
mod utils;

use output::{Output, Profile};

#[derive(Parser)]
struct Args {
    /// The name of the new profile.
    profile: String,

    /// Save the profile in the kanshi configuration directory.
    #[clap(short, long)]
    save: bool,

    /// Save the profile, even if another already exists with the same name.
    #[clap(short, long)]
    force: bool,

    /// The extension of the profile file.
    #[clap(short, long, default_value = "conf")]
    extension: String,

    /// The directory where the profile should be saved [default: $XDG_CONFIG_HOME/kanshi/config.d].
    #[clap(short, long)]
    config_dir: Option<PathBuf>,

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

    let Ok(result) = Command::new("swaymsg").args(["-t", "get_outputs", "--raw"]).output() else {
        eprintln!("Failed to run 'swaymsg -t get_outputs', do you have sway installed?");
        std::process::exit(1);
    };

    let text = String::from_utf8(result.stdout)?;
    let outputs = serde_json::from_str::<Vec<Output>>(&text)?;

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

        let filename = format!("{}.{}", &profile.name, args.extension);
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
