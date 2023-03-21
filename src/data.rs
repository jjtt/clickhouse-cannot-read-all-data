// GENERATED CODE (ch2rs v0.1.6)
#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(warnings)]
#![allow(clippy::all)]

// Generated with the following options:
/*
ch2rs data \
        -T 'String=String' \
        -T 'DateTime64(3, 'UTC')=i64'
*/

#[derive(Debug, clickhouse::Row, serde::Deserialize, serde::Serialize)]
pub struct DataRow {
    pub metadata_id: String,
    pub start_time: i64,
    pub end_time: i64,
    pub double_value: f64,
    pub string_value: String,
    pub long_value: i64,
    pub write_time: i64,
    pub sign: i8,
    pub version: u64,
}
