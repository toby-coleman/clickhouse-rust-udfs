# syntax=docker/dockerfile:1

# Build stage
FROM rust:latest AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

# Final stage
FROM clickhouse/clickhouse-server:latest
# Copy executables
COPY --from=builder /build/target/release /var/lib/clickhouse/user_scripts/

# Copy configs
COPY configs/udfs_function.xml /etc/clickhouse-server/udfs_function.xml
