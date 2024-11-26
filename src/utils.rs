use termion::{color, style};

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