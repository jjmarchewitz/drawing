use cairo::{Context, Format, ImageSurface};
use color_space::Rgb;
use eyre::Result;
use std::fs::File;

pub fn new_context(width: i32, height: i32, color: Rgb) -> Result<(ImageSurface, Context)> {
    let surface = ImageSurface::create(Format::ARgb32, width, height)?;
    let context = Context::new(&surface)?;
    context.set_source_rgb(color.r, color.g, color.b);
    context.paint()?;

    Ok((surface, context))
}

pub fn save_to_png(img: ImageSurface, path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    img.write_to_png(&mut file)?;
    Ok(())
}
