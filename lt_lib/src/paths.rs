use amethyst::utils::application_root_dir;
use std::{env::var_os, path::PathBuf};

#[inline(always)]
pub fn is_end_user_build() -> bool {
    //if cargo_manifest_dir is some, then we know we're developing, because final builds don't have cargo manifest dirs.
    var_os("CARGO_MANIFEST_DIR").is_none()
}

pub fn get_directory(is_config: bool) -> PathBuf {
    let mut path = application_root_dir().unwrap_or_default();
    if !is_end_user_build() {
        path = path.join("../");
    }
    path = if is_config {
        path.join("config")
    } else {
        path.join("assets/data")
    };

    path
}
