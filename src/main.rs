use cairo::{Context, Format, ImageSurface};
use eyre::Result;
use std::fs::File;

fn main() -> Result<()> {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600)?;
    let context = Context::new(&surface)?;
    context.set_source_rgb(1.0, 0.0, 0.0);
    context.paint()?;

    let mut file = File::create("output.png")?;

    surface.write_to_png(&mut file)?;

    Ok(())
}
