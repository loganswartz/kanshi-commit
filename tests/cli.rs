use std::{fs, path::PathBuf, process::Command, sync::LazyLock};
use assert_cmd::cargo::CommandCargoExt;
use assert_fs::{prelude::PathChild, TempDir};

static REPO_ROOT: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
static TEST_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| REPO_ROOT.join("tests").join("data"));
static INPUT_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| TEST_DATA_DIR.join("inputs"));
static OUTPUT_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| TEST_DATA_DIR.join("outputs"));

const DEFAULT_SUBPATH: &str = "kanshi/config.d";

fn make_cmd() -> (Command, TempDir) {
    let tmpdir = assert_fs::TempDir::new().unwrap();
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    command.env("XDG_CONFIG_HOME", tmpdir.as_os_str());

    (command, tmpdir)
}

#[test]
fn location_shown_is_correct() {
    let profile = "test-profile";
    let (mut cmd, tmpdir) = make_cmd();

    let result = cmd.arg(profile).arg("-l").output().expect("command should be runnable");

    let stdout = String::from_utf8(result.stdout).unwrap();
    let expected_path = tmpdir.path().join(DEFAULT_SUBPATH).join(format!("{}{}", profile, ".conf"));
    assert_eq!(stdout.trim(), expected_path.to_str().unwrap());
    assert!(result.status.success());
}

#[test]
fn profile_saved_to_correct_location() {
    let profile = "test-profile";
    let (mut cmd, tmpdir) = make_cmd();

    let expected_path = tmpdir.path().join(DEFAULT_SUBPATH).join(format!("{}{}", profile, ".conf"));
    assert!(std::fs::metadata(&expected_path).is_err(), "file already exists");

    let result = cmd.arg(profile).arg("--from-file").arg(INPUT_DATA_DIR.join("no_outputs.json")).arg("--save").output().expect("command should be runnable");

    assert!(std::fs::metadata(&expected_path).is_ok(), "file was not created");
    assert!(result.status.success());
}

#[test]
fn can_override_the_config_dir() {
    let profile = "test-profile";
    let (mut cmd, tmpdir) = make_cmd();
    let overridden_dir = tmpdir.child("overridden");

    let expected_path = overridden_dir.path().join(format!("{}{}", profile, ".conf"));
    assert!(std::fs::metadata(&expected_path).is_err(), "file already exists");

    let result = cmd.arg(profile).arg("--from-file").arg(INPUT_DATA_DIR.join("no_outputs.json")).arg("--config-dir").arg(overridden_dir.path()).arg("--save").output().expect("command should be runnable");

    assert!(std::fs::metadata(&expected_path).is_ok(), "file was not created");
    assert!(result.status.success());
}

#[test]
fn creates_config_dir_if_it_does_not_exist() {
    let profile = "test-profile";
    let (mut cmd, tmpdir) = make_cmd();
    let non_existent_dir = tmpdir.child("non-existent");

    let expected_path = non_existent_dir.path().join(format!("{}{}", profile, ".conf"));
    assert!(std::fs::metadata(&expected_path).is_err(), "file already exists");

    let result = cmd.arg(profile).arg("--from-file").arg(INPUT_DATA_DIR.join("no_outputs.json")).arg("--config-dir").arg(non_existent_dir.path()).arg("--save").output().expect("command should be runnable");

    assert!(std::fs::metadata(&expected_path).is_ok(), "file was not created");
    assert!(result.status.success());
}

/// Generate a test that reads the input file and compares the output to an expected output.
///
/// The `$name` ident is used as the test name. The input file is expected to be in the
/// `tests/data/inputs` directory with the name `$name.json`. The expected output is expected to be
/// in the `tests/data/outputs` directory with the name `$name.conf`.
macro_rules! input_output_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let profile = stringify!($name);
            let (mut cmd, _) = make_cmd();

            let result = cmd.arg(profile).arg("--from-file").arg(INPUT_DATA_DIR.join(format!("{}{}", profile, ".json"))).output().expect("command should be runnable");

            let stdout = String::from_utf8(result.stdout).unwrap();
            let expected = fs::read_to_string(OUTPUT_DATA_DIR.join(format!("{}{}", profile, ".conf"))).expect("expected output file should exist");

            assert_eq!(stdout.trim(), expected.trim());
            assert!(result.status.success());
        }
    };
}

input_output_test!(single_embedded_output);
input_output_test!(multiple_outputs);
