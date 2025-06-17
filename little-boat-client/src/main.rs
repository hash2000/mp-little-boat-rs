
slint::include_modules!();

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = ApplicationWindow::new()?;



    ui.run()?;

    Ok(())
}
