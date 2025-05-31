use super::{Result, cmd};

pub fn sync() -> Result<()> {
    cmd!("nx", "build").run()?;

    cmd!("nx", "clean").run()?;

    Ok(())
}
