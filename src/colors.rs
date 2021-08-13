use std::fmt;

type Color = (i32, i32, i32);

#[derive(Debug)]
struct KewlColor(Color);

pub fn colors() {
    const COLOR_BLUE: (i32, i32, i32) = (0, 0, 255); // rgb
    println!("Blue (constant): {:?}", COLOR_BLUE);

    let blue = KewlColor((0, 0, 255));
    println!("Blue1: {}", blue);

    let blue2 = Color2 { r: 0, g: 0, b: 255 }; // explain this init form is only private
    println!("Blue2: {:?}", blue2);

    let blue2 = Color2::new(0, 0, 255);
    println!("Blue3: {:?}", blue2);
}

impl fmt::Display for KewlColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Color2 {
    r: u8,
    g: u8,
    b: u8,
}

impl Color2 {
    fn new(r: u8, g: u8, b: u8) -> Color2 {
        return Color2 { r, g, b };
    }
}
