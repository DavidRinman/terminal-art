use term_size;
use image::{imageops, DynamicImage};
use clap::{command, Arg, ArgAction};

mod generators;
use font_rastering;

const CHAR_ASPECT: f32 = font_rastering::W as f32 / font_rastering::H as f32;

fn main() {
  let matches = command!()
    .arg(Arg::new("FILENAME")
      .required(true))

    .arg(Arg::new("invert")
      .short('i')
      .long("invert")
      .help("Inverts the output")
      .action(ArgAction::SetTrue))

    .arg(Arg::new("scale")
      .short('s')
      .long("scale")
      .help("Scale of output relative to console width")
      .value_parser(clap::value_parser!(f32))
      .takes_value(true))

    .arg(Arg::new("mode")
      .short('m')
      .long("mode")
      .help("Method used for rendering")
      .value_parser(["luma", "conv"])
      .default_value("luma"))
    .get_matches();

  let imgpath = matches.get_one::<String>("FILENAME").unwrap();
  let mut img: DynamicImage;
  match image::open(imgpath) {
    Ok(i) => img = i,
    Err(_) => print_nofile(),
  };

  if *matches.get_one::<bool>("invert").unwrap() {
    img.invert();
  }
  
  let scale: f32 = match matches.get_one::<f32>("scale") {
    Some(i) => {
      if i > &0f32 {
        *i
      } else {
        1.0f32
      }
    }
    None => 1.0f32
  };
  let mode: &str = matches.get_one::<String>("mode").unwrap().as_str();
  
  let (term_w, _) = term_size::dimensions().unwrap();
  let ascii_img: String;

  match mode {
    "luma" => {
      let new_w: u32 = (term_w as f32 * scale) as u32;
      let new_h: u32 = (img.height() as f32 / img.width() as f32 * new_w as f32 * CHAR_ASPECT) as u32;
      img = img.resize_exact(new_w, new_h, imageops::FilterType::Triangle);
  
      let img_gray = imageops::colorops::grayscale(&img);  
      ascii_img = generators::generate_luma_ascii(img_gray);
    },
    "conv" => {
      let new_w: u32 = (term_w as f32 * font_rastering::W as f32 * scale) as u32;
      let new_h: u32 = (img.height() as f32 / img.width() as f32 * new_w as f32) as u32;
      img = img.resize_exact(new_w, new_h, imageops::FilterType::Triangle);

      let img_gray = imageops::colorops::grayscale(&img);
      ascii_img = generators::generate_convolution_ascii(img_gray);
    },
    &_ => happy_compiler()
  } 
  println!("{}", ascii_img);
}

fn happy_compiler() -> ! {
  println!("If this error message prints I blame clap");
  std::process::exit(20);
}

fn print_nofile() -> ! {
  println!("Error: No such file or directory");
  std::process::exit(2);
}