use std::error::Error;

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgb};


fn combine_pixel (Luma([i]): Luma<u8>, Luma([n]): Luma<u8>) -> Rgb<u8> {
    let (i, n) = (i as f32 / 255.0, n as f32 / 255.0);
    let random_weight = 0.15;

    let combine = (i * (1.0 - random_weight)) + n * (random_weight);

    match combine {
        | x if x > 0.8 => Rgb([223, 212, 206]),
        | x if x > 0.5 => Rgb([176, 195, 201]),
        x if x > 0.25 =>   Rgb([40, 93,  111]),
        | _ =>                Rgb([14, 32,  36])
    }
}

fn process_image(input: DynamicImage, noise: DynamicImage) -> DynamicImage {
  let (width, height) = (input.width(), input.height());
  let mut new_buffer = ImageBuffer::new(width, height);

  for (pixel_out, (_x, _y, pixel_in)) in new_buffer.pixels_mut().zip(input.pixels()) {
      *pixel_out = pixel_in.to_rgb();
  }
    
  for (x, y, pixel) in new_buffer.enumerate_pixels_mut() {
      //let Luma(luma) = input.get_pixel(x, y).to_luma();
      let noise = noise.get_pixel(x % noise.width(), y % noise.height()).to_luma();
      let out = combine_pixel(pixel.to_luma(), noise);
      *pixel = out;
  }

  DynamicImage::ImageRgb8(new_buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_image = image::open("./resources/waves.jpg")?;
    let noise = image::open("./resources/LDR_LLL1_0.png")?;
    let output_image = process_image(input_image, noise);
    output_image.save("out/stipled.png")?;
    Ok(())
}