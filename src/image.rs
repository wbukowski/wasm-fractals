use std::fs::File;
use std::io::Write;

// Część 1.

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0;width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        assert_eq!(x < self.width, true);
        assert_eq!(y < self.height, true);

        let pixel = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        self.pixels[y*self.width + x] = pixel;
    }

    pub fn to_file(&self, s: &str) {
        let mut file = File::create(s).expect("Couldn't create a file!");
        write!(file, "{}",self.to_string()).expect("Couldn't write to a file!");
    }

    fn to_string(&self) -> String {
        let mut result = String::from("P3\n# Generated ppm\n");
        result += &format!("{} {}\n255\n",self.width, self.height)[..];
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixels[y*self.width + x];
                let r = (pixel >> 16) as u8;
                let g = (pixel >> 8) as u8;
                let b = pixel as u8;
                result += &format!("{} {} {}\t", r, g, b)[..];
            }
            result += "\n";
        }
        result
    }
}