FROM rust:1.83-bookworm as builder

WORKDIR /user/src/app

COPY . .

RUN cargo build --release

FROM debian:bookworm

WORKDIR /app
COPY --from=builder /user/src/app/target/release/simple-nav /app/simple-nav
COPY --from=builder /user/src/app/web /app/web
COPY --from=builder /user/src/app/data /app/data

ENTRYPOINT [ "/app/simple-nav", "/app/data/delaware-latest.osm.pbf" ]