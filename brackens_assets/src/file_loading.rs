//===============================================================

use anyhow::Result;

//===============================================================

pub fn load_string(file_name: &str) -> Result<String> {
    let path = std::path::Path::new("res/").join(file_name);
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

//===============================================================
