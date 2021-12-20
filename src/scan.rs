use std::path::Path;

use glob::glob_with;

/* Scan directory /etc/sysconfig/network-scripts for ifcfg files */
pub fn config_dir(config_dir: &Path) -> Option<Vec<String>> {
    let glob_options = glob::MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let glob_pattern = config_dir.to_str()?.to_owned() + "/ifcfg-*";

    let mut config_paths = vec![];

    for entry in glob_with(&glob_pattern, glob_options).unwrap() {
        match entry {
            Ok(path) => {
                config_paths.push(path.to_str()?.to_owned());
            }
            Err(_err) => continue,
        };
    }

    if !config_paths.is_empty() {
        Some(config_paths)
    } else {
        None
    }
}
