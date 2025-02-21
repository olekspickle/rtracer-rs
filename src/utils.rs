use termion::{color, style};
use image::DynamicImage;
use std::path::Path;

#[macro_export]
#[cfg(feature = "std")]
macro_rules! assert_ok {
	( $x:expr ) => {
		assert_eq!($x, Ok(()));
	};
	( $x:expr, $y:expr ) => {
		assert_eq!($x, Ok($y));
	}
}

pub fn print_green(s: &str) {
	println!("{}{}{}", color::Fg(color::Green), s, color::Fg(color::Reset))
}

pub fn print_italic(s: &str) {
	println!("{}{}{}", style::Italic, s, style::Reset);
}

pub fn save_image(img: DynamicImage, p: &Path) {
    print_italic(&format!("saving as {:?}...", p));

    match img.save(p) {
        Ok(_) => print_green("success!"),
        Err(err) => println!("failed to save {:?}", err),
    }
}