# build
FROM rust:bullseye as builder
ADD . /moete-build
WORKDIR /moete-build
RUN cargo build --release --features "macros"

# runtime
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates

# dependencies for plotters
RUN apt-get install -y libfontconfig1 libfontconfig1-dev
RUN apt-get install -y libfreetype6 libfreetype6-dev

RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /moete-build/target/release/moete /usr/local/bin/moete
CMD ["moete"]