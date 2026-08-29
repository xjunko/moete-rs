# build
FROM rust:bullseye AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
ADD . /moete-build
WORKDIR /moete-build
RUN cargo build --release --features "macros"

# get yt-dlp
FROM debian:bullseye-slim AS ytdlp-fetcher

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux -o /yt-dlp \
    && chmod a+rx /yt-dlp

# get deno
FROM debian:bullseye-slim AS deno-fetcher

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L https://github.com/denoland/deno/releases/latest/download/deno-x86_64-unknown-linux-gnu.zip -o /deno.zip \
    && unzip /deno.zip -d / \
    && chmod a+rx /deno

# runtime
FROM debian:bullseye-slim AS runtime

# dependencies for plotters
RUN apt-get update && apt-get install -y --no-install-recommends \
    libfontconfig1 \
    libfontconfig1-dev \
    libfreetype6 \
    libfreetype6-dev \
    && rm -rf /var/lib/apt/lists/*

# dependencies for music (songbird)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libopus0 \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# final
COPY --from=builder /moete-build/target/release/moete /usr/local/bin/moete
COPY --from=ytdlp-fetcher /yt-dlp /usr/local/bin/yt-dlp
COPY --from=deno-fetcher /deno /usr/local/bin/deno

CMD ["moete"]