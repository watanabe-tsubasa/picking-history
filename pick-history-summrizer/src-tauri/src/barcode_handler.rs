use barcoders::sym::code128::Code128;
use image::{ImageBuffer, Luma};

pub fn process_barcode(jancode: &str, height: i32) -> Result<(String, u32), String> {
  let code_with_charset = format!("Ɓ{}", jancode);
  let barcode = Code128::new(&code_with_charset).map_err(|e| e.to_string())?;
  let png_data = barcode.encode();

  let width = png_data.len() as u32;

  let img = ImageBuffer::from_fn(width, height as u32, |x, _| {
    if png_data[x as usize] == 1 {
      Luma([0u8])
    } else {
      Luma([255u8])
    }
  });

  let output_path = format!("barcode_{}.png", jancode);
  img.save(&output_path).map_err(|e| e.to_string())?;

  Ok((output_path, width))
}