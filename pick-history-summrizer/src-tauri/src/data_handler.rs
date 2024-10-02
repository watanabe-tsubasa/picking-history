use crate::barcode_handler::process_barcode;
use polars::prelude::*;
use polars_excel_writer::PolarsXlsxWriter;
use rust_xlsxwriter::{Image, Workbook};
use calamine::{open_workbook, Reader, Xlsx};
// use std::fs::File;

#[tauri::command]
pub fn process_excel(input_file: &str, output_file: &str) -> Result<(), String> {
  // println!("{}, {}", input_file, output_file);
  let mut excel: Xlsx<_> = open_workbook(input_file).unwrap();
  let sheet_names = excel.sheet_names().to_owned();
  if sheet_names.is_empty() {
    return Err("シートが存在していません".to_string());
  }
  let first_sheet = &sheet_names[0];
  let range = match excel.worksheet_range(first_sheet) {
    Ok(range) => range,
    Err(e) => return Err(format!("ファイルの読み込みに失敗しました: {:?}", e)),
  };

  let mut records: Vec<Vec<String>> = Vec::new();
  for row in range.rows() {
    let record: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
    records.push(record);
  }

  let mut columns = vec![];

  for i in 0..records[0].len() {
    let column_name = &records[0][i];
    let column: Vec<&str> = records.iter().skip(1).map(|r| r[i].as_str()).collect();
    columns.push(Series::new(column_name.into(), column));
  }
  // println!("columns: {:?}", columns);
  
  let df = DataFrame::new(columns).map_err(|e| e.to_string())?;
  // println!("df: {:?}", df);

  let df_processed = df
    .lazy()
    .select([
      col("外部注文番号"),
      col("荷受け人"),
      col("備考"),
      col("商品コード"),
      col("商品名称"),
      col("ユーザー購入数量"),
      col("ピッキング数量"),
      col("欠品数量"),
      col("代替品数量"),
      col("商品価格"),
      col("商品ステータス"),
      col("代替商品コード"),
      col("代替商品名称"),
      col("明細修正"),
    ])
    .filter(col("明細修正").eq(lit("有")))
    .collect()
    .map_err(|e| e.to_string())?;
  // println!("{:?}", df_processed);

  let product_codes = df_processed.column("商品コード").map_err(|e| e.to_string())?;
  let barcode_images: Vec<Option<String>> = vec![None; df_processed.height()];
  let barcode_series = Series::new("バーコード".into(), barcode_images);
  let df_with_barcode = df_processed.hstack(&[barcode_series]).map_err(|e| e.to_string())?;

  let mut workbook = Workbook::new();
  let mut worksheet = workbook.add_worksheet();

  let mut xlsx_writer = PolarsXlsxWriter::new();
  xlsx_writer.write_dataframe_to_worksheet(&df_with_barcode, &mut worksheet, 0, 0).map_err(|e| e.to_string())?;

  for (i, code) in product_codes.iter().enumerate() {
    let code_str = code.to_string();
    let jancode = code_str.trim_matches('"');
    if !jancode.is_ascii() {
      println!("Skipping non-ASCII 商品コード: {}", jancode);
      continue;
    }
    let image_height = 80; // 画像の高さ（ピクセル）
    let (barcode_path, image_width) = process_barcode(&jancode, image_height)?;
    let image = Image::new(&barcode_path).map_err(|e| e.to_string())?;
    let width = df_processed.width() as u16;

    worksheet.set_column_width(width, (image_width / 7) as f64).map_err(|e| e.to_string())?;
    worksheet.set_row_height((i + 1) as u32, image_height as f64).map_err(|e| e.to_string())?;  
    worksheet.insert_image((i + 1) as u32, width, &image).map_err(|e| e.to_string())?;

    // barcode_pathの画像を削除
    std::fs::remove_file(&barcode_path).map_err(|e| e.to_string())?;
  }
  
  workbook.save(output_file).map_err(|e| e.to_string())?;
  
  // polars_excel_writer::ExcelWriter::new(&mut file).finish(&mut df).map_err(|e| e.to_string())?;
  // let mut file = File::create(output_file).map_err(|e| e.to_string())?;
  // polars_excel_writer::ExcelWriter::new(&mut file).finish(&mut df).map_err(|e| e.to_string())?;

  Ok(())
}