use image::{DynamicImage, ImageFormat};
use std::fs::{File, OpenOptions};
use std::path::Path;
use termion::{color, style};

#[macro_export]
#[cfg(feature = "std")]
macro_rules! assert_ok {
    ( $x:expr ) => {
        assert_eq!($x, Ok(()));
    };
    ( $x:expr, $y:expr ) => {
        assert_eq!($x, Ok($y));
    };
}

pub fn print_green(s: &str) {
    println!(
        "{}{}{}",
        color::Fg(color::Green),
        s,
        color::Fg(color::Reset)
    )
}

pub fn print_italic(s: &str) {
    println!("{}{}{}", style::Italic, s, style::Reset);
}

pub fn save_image(img: DynamicImage, path: &Path) {
    print_italic(&format!("saving as {:?}...", path));

    let mut _image_file: File = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();

    match img.save_with_format(path, ImageFormat::Png) {
        Ok(_) => print_green("success!"),
        Err(err) => println!("failed to save {:?}", err),
    }
}
