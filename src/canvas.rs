use super::color::*;
use std::fmt::Write;
use std::ops::{Index, IndexMut};
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Box<[Color]>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::default(); width * height].into_boxed_slice(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
        &self[self.width * y + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let width = self.width;
        self[(width * y + x) as usize] = color;
    }

    pub fn to_ppm(&self) -> String {
        let mut buf = String::new();

        // header
        buf += "P3\n";
        write!(buf, "{} {}\n", self.width, self.height).unwrap();
        write!(buf, "255\n").unwrap();

        // body
        for y in 0..self.height {
            let row_start_index = y * self.width;
            let row_end_index = row_start_index + self.width;
            let row = &self.pixels[row_start_index..row_end_index];
        }

        

        buf
    }
}

impl Index<usize> for Canvas {
    type Output = Color;

    fn index(&self, i: usize) -> &Color {
        &self.pixels[i]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, i: usize) -> &mut Color {
        &mut self.pixels[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_blank_canvas() {
        let canvas = Canvas::new(10, 20);
        assert!(canvas.width == 10);
        assert!(canvas.height == 20);

        for pixel in canvas.pixels.iter() {
            assert!(*pixel == Color(0.0, 0.0, 0.0))
        }
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);

        assert!(*canvas.get_pixel(2, 3) == red)
    }

    #[test]
    fn construct_ppm_header() {
        let canvas = Canvas::new(10, 20);
        let ppm = canvas.to_ppm();

        let lines: Vec<&str> = ppm.split('\n').collect();

        assert!(lines[0] == "P3");
        assert!(lines[1] == "10 20");
        assert!(lines[2] == "255");
    }

    #[test]
    fn split_ppm_into_lines() {
        let mut canvas = Canvas::new(10, 20);
        for i in 0..canvas.size() {
            canvas[i] = Color(1.0, 0.8, 0.6)
        }

        let ppm = canvas.to_ppm();

        let lines: Vec<&str> = ppm.split('\n').collect();

        println!("{:?}", lines[2]);
        assert!(lines[0] == "P3");
        assert!(lines[1] == "10 20");
        assert!(lines[2] == "255");
    }
}
