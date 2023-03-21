mod data;

use anyhow::Result;
use crate::data::DataRow;
use clickhouse::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let client = Client::default().with_url("http://localhost:8124");
    let mut inserter = client.inserter("data")?.with_max_entries(1000);
    let row = DataRow {
        metadata_id: "".to_string(),
        start_time: 0,
        end_time: None,
        double_value: 0.0,
        string_value: "".to_string(),
        long_value: 0,
        write_time: 0,
        sign: 0,
        version: 0,
    };
    inserter.write(&row).await.unwrap();
    inserter.commit().await.unwrap();
    inserter.end().await.unwrap();
    Ok(())
}
