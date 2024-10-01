use crate::barcode_handler::process_barcode;
use polars::prelude::*;
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

  process_barcode("jancode");
  
  let mut xlsx_writer = polars_excel_writer::PolarsXlsxWriter::new();
  
  xlsx_writer.write_dataframe(&df_processed).map_err(|e| e.to_string())?;
  xlsx_writer.save(output_file).map_err(|e| e.to_string())?;
  
  // polars_excel_writer::ExcelWriter::new(&mut file).finish(&mut df).map_err(|e| e.to_string())?;
  // let mut file = File::create(output_file).map_err(|e| e.to_string())?;
  // polars_excel_writer::ExcelWriter::new(&mut file).finish(&mut df).map_err(|e| e.to_string())?;

  Ok(())
}