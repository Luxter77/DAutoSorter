use std::io;

fn main() -> io::Result<()> {
    #[cfg(windows)] {
        winres::WindowsResource::new().set_icon("assets/s-l300.ico").compile()?;
    }
    Ok(())
}