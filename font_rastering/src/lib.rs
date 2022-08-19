use fontdue;

pub const W: usize = 8;
pub const H: usize = 16;
const ORIGIN_X: i32 = 1;
const ORIGIN_Y: i32 = 4;

pub fn rasterize(c: char) -> [[u8; W]; H] {
  let font = include_bytes!("../resources/UbuntuMono-R.ttf") as &[u8];
  let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
  let (metrics, bitmap) = font.rasterize(c, 12.0);

  let start_ix = (metrics.xmin + ORIGIN_X) as usize;
  let start_iy = (metrics.ymin + ORIGIN_Y) as usize;
  let mut raster = [[0u8; W]; H];
  let mut i: usize = 0;

  for y in start_iy..(start_iy + metrics.height) {
    for x in start_ix..(start_ix + metrics.width) {
      raster[y][x] = bitmap[i];
      i = i + 1;
    }
  }
  return raster;
}
