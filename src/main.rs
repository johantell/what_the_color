mod color;

use exitfailure::ExitFailure;
use std::env;

fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();

    if args.get(1).is_none() {
        eprintln!("No input provided.\n\nUsage: `findcolor [HEX COLOR]`");

        return Ok(());
    }

    let hex_code = args.get(1).unwrap();

    match color::parse_hex(hex_code) {
        Ok(rgb) => {
            let color = color::find_closest_match(&rgb);
            color::description_printer::print_details(color, rgb);
        }
        Err(_) => eprintln!("`{}` could not be parsed as an hex color", hex_code),
    };

    return Ok(());
}
