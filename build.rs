use std::{fs, path::Path};

use anyhow::Result;
use fluent_static_codegen::{generate, FunctionPerMessageCodeGenerator};

pub fn main() -> Result<()> {
    let src = generate("i18n", FunctionPerMessageCodeGenerator::new("en-US"))?;
    let destination = Path::new("src/text.rs");

    fs::write(destination, src)?;

    Ok(())
}
