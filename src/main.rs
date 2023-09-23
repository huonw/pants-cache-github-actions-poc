use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
  let service = opendal::services::Ghac::default();
  let op = opendal::Operator::new(service).unwrap().finish();

  // unique for every run to test behaviour:
  let now = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
  let make_key = |s: &str| dbg!(format!("{}-{}", s, now.as_secs_f64()));

  println!("overwrite");
  let key_rewrite = make_key("rewrite");
  op.write(&key_rewrite, "data").await.unwrap();
  dbg!(op.read(&key_rewrite).await.unwrap().len());

  // write again
  let _ = dbg!(op.write(&key_rewrite, "data").await);
  let _ = dbg!(op.writer(&key_rewrite).await.map(|_| ()));

  println!("from file via tokio::io::copy");
  let key_file_tokio = make_key("file-tokioiocopy");
  let big_data = "data".repeat(10000);
  let big_data = big_data.as_bytes();
  let mut reader = std::io::Cursor::new(big_data);
  let mut writer = op.writer(&key_file_tokio).await.unwrap();

  dbg!(tokio::io::copy(&mut reader, &mut writer).await).unwrap();
  dbg!(writer.close().await).unwrap();
  // what was written?
  dbg!(op.read(&key_file_tokio).await.unwrap().len());

  println!("from file via sink");
  let key_file_writer = make_key("file-writersink");
  let stream = futures::stream::repeat_with(|| Ok(bytes::Bytes::from("data"))).take(10);
  let mut writer = op.writer(&key_file_writer).await.unwrap();

  dbg!(writer.sink(stream).await).unwrap();
  dbg!(writer.close().await).unwrap();
  // what was written?
  dbg!(op.read(&key_file_writer).await.unwrap().len());
}
