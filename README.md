# Rust UDFs for ClickHouse

This repo demonstrates a way to implement [ClickHouse user-defined functions (UDFs)](https://clickhouse.com/docs/sql-reference/functions/udf) in Rust. In particular, these UDFs use the [`RowBinary`](https://clickhouse.com/docs/interfaces/formats/RowBinary) format to exchange data between ClickHouse and the executable, which ensures that `DateTime` objects are sent as UNIX timestamps, i.e. are in UTC.

## Contents

This repo contains the following functions.

### GB electricity market

Functions for [settlement window calculations](https://www.elexon.co.uk/bsc/settlement/) in the GB electricity market.

| Function                       | Description                                             | Args            |
|------------------------------- | ------------------------------------------------------- | --------------- |
| `dateTimeToSettlementPeriod`   | Converts a timestamp into a settlement period           | `DateTime`      |
| `dateTimeToSettlementDate`     | Converts a timestamp to its associated settlement day   | `DateTime`      |
| `settlementPeriodToDateTime`   | Converts a date and settlement period to a UTC datetime | `Date`, `UInt8` |


## Running in Docker

To try out the function in a Docker environment, run:
```sh
docker build -t clickhouse-udfs .
docker run -d --name my-clickhouse-server --ulimit nofile=262144:262144 clickhouse-udfs
docker exec -it my-clickhouse-server clickhouse-client
```

This will enter into a `clickhouse-client` session, from which you can test the UDFs on the server, e.g.
```sql
SELECT dt,
    dateTimeToSettlementDate(dt) AS settle_dt,
    dateTimeToSettlementPeriod(dt) AS sp,
    settlementPeriodToDateTime(settle_dt, sp) AS recovered
FROM generateRandom('dt DateTime')
LIMIT 10;
```

## How it works

Each binary will read binary data from ClickHouse via stdin. When it receives enough bytes for all the input args, it will compute the output and send it to stdout. If ClickHouse send EOF, this indicates that the process can exit.
