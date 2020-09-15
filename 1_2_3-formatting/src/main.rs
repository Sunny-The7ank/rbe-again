/*
####### NOTES #######
We've seen that formatting is specified via a format string: 
    format!("{}", foo) -> "3735928559"
    format!("0x{:X}", foo) -> "0xDEADBEEF"
    format!("0o{:o}", foo) -> "0o33653337357"

The same variable (foo) can be formatted differently depending on argument type 
used: X vs o vs unspecified

This formatting functionality is implemented via traits, and there is one trait
for each argument type. The most common formatting trait is Display, which handles
cases where arg type is left unspecified {} for instance
*/

use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}",self.red, self.green, self.blue)
    }
}

// This is the more correct response
impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

// Solution seen elsewhere
/*
impl Display for Color {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // RGB (128, 255, 9) 0x80FF5A  0x{:>02X}{:>02X}{:>02X}
        // I am not very happy with this solution. I think this can be done in a better way.
        write!(f, "RGB ({0}, {1}, {2}) 0x{3}{4}{5}",
               self.red, self.green, self.blue,
               format!("{:01$x}", self.red, 2),
               format!("{:01$x}", self.green, 2),
               format!("{:01$x}", self.blue, 2))
    }
}

I don't like this because {:X} can just be used to call upper hex
*/

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("RGB ({}) 0x{:X}", *color, *color);
    }
}