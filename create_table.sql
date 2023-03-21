CREATE TABLE IF NOT EXISTS data (
    metadata_id String,
    start_time DateTime64(3, 'UTC') CODEC(DoubleDelta, ZSTD(22)),
    end_time DateTime64(3, 'UTC') CODEC(DoubleDelta, ZSTD(22)),

    double_value Float64 CODEC(DoubleDelta, ZSTD(22)),
    string_value String CODEC(ZSTD(22)),
    long_value Int64,

    write_time DateTime64(3, 'UTC') CODEC(DoubleDelta, ZSTD(22)),
    sign Int8 DEFAULT 1,
    version UInt64
) Engine = VersionedCollapsingMergeTree(sign, version)
partition by toYYYYMM(start_time)
ORDER BY (metadata_id, start_time, end_time);
