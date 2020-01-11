extern crate cargo_msrv;

use cargo_msrv::fetch::RustStableVersion;
use indicatif::ProgressBar;
use std::ffi::OsString;
use std::path::PathBuf;

#[test]
fn validate_feature_versions() {
    check_all_feature_versions(with_path);
}

#[test]
fn validate_feature_versions_with_custom_cmd() {
    fn f(path: PathBuf) -> impl IntoIterator<Item = String> {
        let path_args = with_path(path).into_iter();
        let custom_check_args: Vec<String> =
            vec!["--".to_string(), "cargo".to_string(), "check".to_string()];

        path_args.chain(custom_check_args)
    }

    check_all_feature_versions(f)
}

fn check_all_feature_versions<I: IntoIterator<Item = T>, T: Into<OsString> + Clone>(
    args: impl Fn(PathBuf) -> I,
) {
    let features_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("features");

    for path in std::fs::read_dir(features_path).unwrap() {
        let path = path.unwrap();
        let meta = &path.metadata().unwrap();
        if meta.is_dir() {
            let project_dir = path.path();
            println!("dir: {:?}", &project_dir);

            let matches = cargo_msrv::cli::cli().get_matches_from(args(project_dir.clone()));
            let matches = cargo_msrv::cli::cmd_matches(&matches).unwrap();
            println!("matches: {:?}", &matches);

            let result = cargo_msrv::msrv(
                &matches,
                RustStableVersion::new(1, 38, 0),
                &ProgressBar::new_spinner(),
            )
            .unwrap();
            println!("result: {:?}", &result);

            let expected = project_dir.clone();
            let expected = expected.iter().last().unwrap();
            let expected = expected.to_str().unwrap();
            let expected =
                RustStableVersion::from_parts(&expected.split('.').collect::<Vec<_>>()).unwrap();

            assert_eq!(result.unwrap(), expected);
        }
    }
}

fn with_path(path: PathBuf) -> impl IntoIterator<Item = String> {
    let path = path.as_os_str().to_string_lossy().to_string();

    let args = format!("cargo msrv --path {}", path);
    let args = args
        .split_ascii_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    args
}