use crate::color::rgb::RGB;
use crate::color::{Color, ColorConvertable};
use colored::*;

pub fn print_details(color: &Color, passed_color: RGB) {
    let rgb_value = color.rgb();

    println!(
        "{} {} {}",
        "   ".on_truecolor(rgb_value.0, rgb_value.1, rgb_value.2),
        color.name.bold(),
        if rgb_value == passed_color {
            " "
        } else {
            "(Approx.)"
        }
    );
    println!("");
    println!("hex: {}", color.hex);
    println!("rgb: {:?}", rgb_value);
    println!("");
    println!("{}", parameterize(color.name).bold());
}

fn parameterize(string: &str) -> String {
    return string.to_lowercase().replace("/", "").replace(" ", "-");
}
