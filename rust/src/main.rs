extern crate exitcode;

use std::env::args;

pub mod args;
pub mod color;
pub mod config;
pub mod debug;
pub mod font;
pub mod helpers;
pub mod render;

use debug::{d, Dt};
use render::render;

fn main() {
	let options = args::parse(args().collect::<Vec<String>>());
	d("main()", 1, Dt::Head, &options, &mut std::io::stdout());
	d(
		&format!("main() CLI args parsed from\n{:#?}\nto:\n{:#?}", args().collect::<Vec<String>>(), options),
		3,
		Dt::Log,
		&options,
		&mut std::io::stdout(),
	);

	if options.version {
		// show version
		return;
	}

	if options.help {
		// show help
		return;
	}

	let render_options = render(&options);
	println!("{}", render_options.text);
}
