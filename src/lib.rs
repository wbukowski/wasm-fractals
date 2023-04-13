mod utils;
mod image;
mod complex;

use complex::Complex;
use image::Image;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    let alert_message = &format!("Hello, {}!", s)[..];
    alert(alert_message);
}

#[wasm_bindgen]
pub struct Fractal {
    pub width: usize,
    pub height: usize,
    img: Image
}

fn mandelbrot(c: Complex) -> u8 {
    let mut z = c;
    for i in 0..255 {
        z = z * z + c;
        if z.real.abs() > 2.0 || z.imaginary.abs() > 2.0 {
            return i
        }
    }
    255
}

#[wasm_bindgen]
impl Fractal {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, img: Image::new(width, height) }
    }
    pub fn render(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Vec<u8> {
        let mut res = vec![];
        let delta_x = max_x - min_x;
        let delta_y = max_y - min_y;
        for i in 0..self.img.pixels.len() {
            let x = i % self.width;
            let y = i / self.width;
            let color = mandelbrot(Complex{ real: (x as f64*delta_x) / self.width as f64 + min_x, imaginary: (y as f64*delta_y) / self.height as f64 + min_y });
            //self.img.set_pixel(x, y, 0, 0, color);
            res.push(0);
            res.push(0);
            res.push(color);
            res.push(255);
        }
        // let mut res = vec![];
        // for p in self.img.pixels.iter() {
        //     res.push((p >> 16) as u8);
        //     res.push((p >> 8)  as u8);
        //     res.push(*p         as u8);
        //     res.push(255);
        // }
        res
    }

}