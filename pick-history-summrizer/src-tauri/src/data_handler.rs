use polars::prelude::*;
use calamine::{open_workbook, Reader, Xlsx};
use std::fs::File;

#[tauri::command]
pub fn process_excel(input_file: &str, output_file: &str) -> Result<(), String> {
  let mut excel: Xlsx<_> = open_workbook(input_file).unwrap();
  let range = match excel.worksheet_range("注文商品") {
    Ok(range) => range,
    Err(e) => panic!("Failed to read worksheet: {:?}", e),
  };

  let mut records: Vec<Vec<String>> = Vec::new();
  for row in range.rows() {
    let record: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
    records.push(record);
  }

  let mut columns = vec![];

  for i in 0..records[0].len() {
    let column: Vec<&str> = records.iter().map(|r| r[i].as_str()).collect();
    columns.push(Series::new(format!("column_{}", i).into(), column));
  }

  let mut df = DataFrame::new(columns).map_err(|e| e.to_string())?;

  let mut file = File::create(output_file).map_err(|e| e.to_string())?;
  polars_excel_writer::ExcelWriter::new(&mut file).finish(&mut df).map_err(|e| e.to_string())?;

  Ok(())
}