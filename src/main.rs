use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
  let service = opendal::services::Ghac::default();
  let op = opendal::Operator::new(service).unwrap().finish();

  // unique for every run to test behaviour:
  let now = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();

  let key = format!("key-{}", now.as_secs_f64());
  let writer1 = op.writer(&key).await;
  dbg!(writer1.map(|_| ()));
  let writer2 = op.writer(&key).await;
  dbg!(writer2.map(|_| ()));
}
