//===============================================================

use std::{fs::create_dir, path::Path};

use anyhow::*;
use fs_extra::{copy_items, dir::CopyOptions};

//===============================================================

fn main() -> Result<()> {
    if !Path::new("res/").exists() {
        create_dir("res/")?;
    }

    // This tells cargo to re-run this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=res/*");

    let out_dir = std::env::var("OUT_DIR")?;
    println!("Out dir = {}", out_dir);
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("res/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}

//===============================================================
