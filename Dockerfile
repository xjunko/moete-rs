# build
FROM rust:bookworm AS builder
RUN apt-get update --fix-missing && apt-get install -y --no-install-recommends \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
ADD . /moete-build
WORKDIR /moete-build
RUN cargo build --release --features "macros"

# runtime
FROM debian:bookworm-slim AS runtime

# dependencies for plotters
RUN apt-get update --fix-missing && apt-get install -y --no-install-recommends \
    libfontconfig1 \
    libfontconfig1-dev \
    libfreetype6 \
    libfreetype6-dev \
    && rm -rf /var/lib/apt/lists/*

# dependencies for music (songbird)
RUN apt-get update --fix-missing && apt-get install -y --no-install-recommends \
    ca-certificates \
    libopus0 \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# final
COPY --from=builder /moete-build/target/release/moete /usr/local/bin/moete

CMD ["moete"]