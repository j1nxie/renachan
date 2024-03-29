use crate::color::Color;
use std::{fs::File, io::Write, path::Path};

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> &Self {
        self[(x, y)] = color;
        self
    }

    pub fn write_to_ppm(&self, path: &Path) -> std::io::Result<()> {
        let mut f = File::create(path)?;
        let headers = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut pixels = String::new();

        let mut i = 0;

        for pixel in self.pixels.iter() {
            let pixel_int = pixel.to_int(255);
            pixels.push_str(&format!("{} {} {} ", pixel_int.r, pixel_int.g, pixel_int.b));

            i += 1;

            if i == self.width {
                i = 0;
                pixels.push('\n');
            }
        }

        let mut contents = String::new();
        contents.push_str(&headers);
        contents.push_str(
            &pixels
                .trim()
                .lines()
                .map(|part| part.trim())
                .collect::<Vec<&str>>()
                .join("\n"),
        );
        contents.push('\n');

        match f.write(contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => panic!("error writing to file: {}", e),
        }
    }
}

impl std::ops::Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Color {
        match self.pixels.get(x + y * self.width) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for canvas size ({}, {})",
                x, y, self.width, self.height
            ),
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Color {
        match self.pixels.get_mut(x + y * self.width) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for canvas size ({}, {})",
                x, y, self.width, self.height
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        io::{prelude::*, BufReader},
    };

    #[test]
    fn test_canvas() {
        let mut c = Canvas::new(3, 3);
        let black = Color::new(0.0, 0.0, 0.0);
        let red = Color::new(1.0, 0.0, 0.0);

        assert_eq!(c.width, 3);
        assert_eq!(c.height, 3);

        for pixel in c.pixels.iter() {
            assert_eq!(*pixel, black);
        }

        c.pixels = vec![black, red, black, black, red, black, black, black, black];

        assert_eq!(c[(1, 0)], red);
        assert_eq!(c[(1, 1)], red);
    }

    #[test]
    fn test_write_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let p1 = Color::new(1.0, 2.0, 3.0);
        let p2 = Color::new(2.0, 3.0, 4.0);

        c.write_pixel(3, 4, p1);
        c.write_pixel(6, 9, p2);

        assert_eq!(c[(3, 4)], p1);
        assert_eq!(c[(6, 9)], p2);
    }

    #[test]
    fn test_write_empty_ppm() {
        let c = Canvas::new(5, 3);
        c.write_to_ppm(Path::new("test_write_empty_ppm.ppm"))
            .unwrap();

        let file = File::open("test_write_empty_ppm.ppm").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        assert_eq!(content, "P3\n5 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n");

        fs::remove_file("test_write_empty_ppm.ppm").unwrap();
    }

    #[test]
    fn test_write_ppm() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(1, 0, Color::new(0.0, 0.5, 0.5));
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        println!("{:?}", c.pixels);

        c.write_to_ppm(Path::new("test_write_ppm.ppm")).unwrap();

        let file = File::open("test_write_ppm.ppm").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        assert_eq!(content, "P3\n5 3\n255\n255 0 0 0 128 128 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");

        fs::remove_file("test_write_ppm.ppm").unwrap();
    }
}
