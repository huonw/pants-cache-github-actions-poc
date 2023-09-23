use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
  let service = opendal::services::Ghac::default();
  let op = opendal::Operator::new(service).unwrap().finish();

  // unique for every run to test behaviour:
  let now = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();

  let key = format!("key-{}", now.as_secs_f64());
  let mut writer = op.writer(&key).await.unwrap();

  writer.write_all(&[0]).await.unwrap();
  writer.write_all(&[1]).await.unwrap();
  writer.close().await.unwrap();

  println!("{:?}", op.read(&key).await.unwrap());
}
