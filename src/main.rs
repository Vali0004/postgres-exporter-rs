use postgres::{Client, NoTls};
use std::fs::{File, rename};
use std::io::{Write, BufWriter};
use std::thread::sleep;
use std::time::Duration;

fn do_scrape(client: &mut Client, outname: &String) -> Result<(), Box<dyn std::error::Error>> {
  let tempfile = format!("{}.next", outname);

  let file = File::create(&tempfile)?;
  let mut writer = BufWriter::new(file);

  let rows = client.query(
    "SELECT datname, sessions FROM pg_stat_database WHERE datname IS NOT NULL",
    &[],
  )?;

  for row in rows {
    let dbname: &str = row.get(0);
    let connection_count: i64 = row.get(1);

    writeln!(
      writer,
      "pg_connections_count{{dbname=\"{}\"}} {}",
      dbname,
      connection_count
    )?;
  }

  writer.flush()?;
  rename(&tempfile, outname)?;

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let conn_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "host=localhost user=postgres application_name=postgres-rates".into());

  let output_path = std::env::var("OUTPUT_PATH")
    .unwrap_or_else(|_| "/var/lib/prometheus-node-exporter-text-files/postgres-rates.prom".into());

  let interval: u64 = std::env::var("SCRAPE_INTERVAL")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(60);

  let mut client = postgres::Client::connect(&conn_str, postgres::NoTls)?;

  loop {
    if let Err(e) = do_scrape(&mut client, &output_path) {
      eprintln!("scrape error: {}", e);
    }
    std::thread::sleep(std::time::Duration::from_secs(interval));
  }
}