use std::collections::HashMap;
use std::f32::consts::PI;

use image::{
  ImageBuffer,
  Luma,
};
use font_rastering;

const LUMA_CHARS: &[u8] = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ".as_bytes();
const F: f32 = ( LUMA_CHARS.len() - 1 ) as f32 / 255.0f32;

fn nonlinear(x: f32) -> f32 {
  0.5*(PI*(x - 0.5)).sin() + 0.5
}

fn normalize(img_gray: &mut ImageBuffer<Luma<u8>, Vec<u8>>) {
  let mut min: u8 = 255;
  let mut max: u8 = 0;
  for p in img_gray.pixels() {
    let val = p.0[0];
    if val < min { 
      min = val;
    }
    else if val > max {
      max = val;
    }
  }
  if min != max {
    for p in img_gray.pixels_mut() {
      let nl = nonlinear((p.0[0] - min) as f32 / (max - min) as f32);
      p.0[0] = (nl * 255f32) as u8;
    }
  }
}

pub fn generate_luma_ascii(mut img_gray: ImageBuffer<Luma<u8>, Vec<u8>>) -> String { 
  normalize(&mut img_gray);
  let mut ascii_img: String = String::from("");
  for r in img_gray.rows() {
    let line: String = r.map(
      |p| LUMA_CHARS[((p.0[0] as f32) * F) as usize] as char
    ).collect();
    ascii_img.push_str(&line);
    ascii_img.push_str("\n");
  }
  return ascii_img;
}

// todo: parallellize, maybe with shaders
pub fn generate_convolution_ascii(mut img_gray: ImageBuffer<Luma<u8>, Vec<u8>>) -> String {
  normalize(&mut img_gray);
  let mut ascii_img: String = String::from("");
  let mut filters = HashMap::new();     // Using hashmap here is likely not efficient
  for charcode in 32..127u8 {           // but my goopy python brain likes dictionaries
    let c = charcode as char;         
    if c.is_ascii() {
      let raster = font_rastering::rasterize(c);
      filters.insert(c, raster);
    }
  }

  let (w, h) = img_gray.dimensions();
  let nby = h as usize / font_rastering::H;
  let nbx = w as usize / font_rastering::W;

  for by in 0..nby {
    for bx in 0..nbx {
      let mut top_score: i32 = i32::MIN;
      let mut top_char = ' ';

      for (c, filter) in filters.iter() {
        let mut score: i32 = 0;
        for y in by*font_rastering::H..(by+1)*font_rastering::H {
          for x in bx*font_rastering::W..(bx+1)*font_rastering::W {
            let img_val = img_gray.get_pixel(x as u32, y as u32).0[0] as i32 - 128;
            let filter_val = 128 - filter[y % font_rastering::H][x % font_rastering::W] as i32;
            score = score + img_val * filter_val;
          }
        }
        if score > top_score {
          top_score = score;
          top_char = *c;
        }
      }
      ascii_img.push(top_char);
    }
    ascii_img.push_str("\n");
  }
  return ascii_img;
}