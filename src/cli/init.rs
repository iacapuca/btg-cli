use crate::commands;

pub fn init() -> Result<(), anyhow::Error> {
    commands::init::run()
}
