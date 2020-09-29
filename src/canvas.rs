use super::color::*;
use std::fmt::Write;
use std::ops::{Index, IndexMut};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Box<[Color]>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::default(); width * height].into_boxed_slice(),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
        &self[self.width * y + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let width = self.width;
        self[width * y + x] = color;
    }

    pub fn to_ppm(&self) -> String {
        let mut buf = String::new();

        // header
        buf += "P3\n";
        writeln!(buf, "{} {}", self.width, self.height).unwrap();
        writeln!(buf, "255").unwrap();

        // body
        for y in 0..self.height {
            let row_start_index = y * self.width;
            let row_end_index = row_start_index + self.width;
            let row = &self.pixels[row_start_index..row_end_index];
            let mut line = String::new();
            for (i, color) in row.iter().enumerate() {
                write!(
                    line,
                    "{} {} {}",
                    (color.0 * 255.0).min(255.0).max(0.0).round() as i32,
                    (color.1 * 255.0).min(255.0).max(0.0).round() as i32,
                    (color.2 * 255.0).min(255.0).max(0.0).round() as i32
                )
                .unwrap();

                if i < self.width - 1 {
                    write!(line, " ").unwrap();
                }
            }
            writeln!(line).unwrap();
            write!(buf, "{}", line).unwrap();
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

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
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
    fn construct_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color(1.5, 0.0, 0.0);
        let c2 = Color(0.0, 0.5, 0.0);
        let c3 = Color(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let ppm = canvas.to_ppm();

        let lines: Vec<&str> = ppm.split('\n').collect();

        assert!(lines[3] == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert!(lines[4] == "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert!(lines[5] == "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    // #[test]
    // fn split_ppm_lines_at_70_chars() {
    //     let mut canvas = Canvas::new(5, 3);

    //     for y in 0..canvas.height {
    //         for x in 0..canvas.width {
    //             canvas.write_pixel(x, y, Color(1.0, 0.6, 0.8));
    //         }
    //     }

    //     let ppm = canvas.to_ppm();

    //     let lines: Vec<&str> = ppm.split('\n').collect();

    //     assert!(lines[3] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
    //     assert!(lines[4] == "153 255 204 153 255 204 153 255 204 153 255 204 153");
    //     assert!(lines[5] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
    //     assert!(lines[6] == "153 255 204 153 255 204 153 255 204 153 255 204 153");
    // }

    #[test]
    fn ppm_termintated_with_newline() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();

        assert!(ppm.chars().last().unwrap() == '\n')
    }
}
