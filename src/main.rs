mod util;

use color_space::{Hsl, Hsv, Rgb};
use coordinates::prelude::*;
use eyre::Result;

fn main() -> Result<()> {
    let background = Rgb::from_hex(0xFFFFFF);
    let width = 600;
    let height = 600;
    let (surface, context) = util::new_context(width, height, background)?;

    let mut radius = 300.;
    let mut theta = 0.;
    let line_length = 25.;
    let mut color = Rgb::from_hex(0x000000);

    for _ in 1..15_000 {
        let outer_point = Polar::<f64> { radius, theta };
        let inner_point = Polar::<f64> {
            radius: radius - line_length,
            theta,
        };

        let mut outer_point = Vector2::from(outer_point);
        outer_point.x += width as f64 / 2.;
        outer_point.y += height as f64 / 2.;

        let mut inner_point = Vector2::from(inner_point);
        inner_point.x += width as f64 / 2.;
        inner_point.y += height as f64 / 2.;

        // let color

        context.set_source_rgb(0., 0., 0.);
        context.move_to(outer_point.x, outer_point.y);
        context.line_to(inner_point.x, inner_point.y);
        context.stroke()?;

        radius -= 0.02;
        theta += 0.01;
        theta %= 360.;

        if radius <= 0. {
            break;
        }
    }

    util::save_to_png(surface, "images/output.png")?;
    Ok(())
}
