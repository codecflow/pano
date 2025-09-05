# Multi-architecture Dockerfile for glibc-based builds
FROM --platform=$BUILDPLATFORM rust:bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Compute Rust target and set up cross-compilation tools
RUN <<EOF
case "${TARGETPLATFORM}" in
    "linux/amd64") RUST_TARGET="x86_64-unknown-linux-gnu" ;;
    "linux/arm64") RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    *) echo "Unsupported platform: ${TARGETPLATFORM}" && exit 1 ;;
esac

echo "Building for ${RUST_TARGET}"
echo "export RUST_TARGET=${RUST_TARGET}" >> /etc/environment

rustup target add ${RUST_TARGET}

if [ "${RUST_TARGET}" = "aarch64-unknown-linux-gnu" ]; then
    apt-get update && apt-get install -y gcc-aarch64-linux-gnu
fi
EOF

# Install GUI dependencies
RUN <<EOF
apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential \
    pkg-config \
    ca-certificates \
    libglib2.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libgdk-pixbuf2.0-dev \
    libatk1.0-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libx11-dev \
    libxext-dev \
    libxrender-dev \
    libxi-dev \
    libxrandr-dev \
    libxcursor-dev \
    libxcomposite-dev \
    libxdamage-dev \
    libxfixes-dev \
    libxss-dev \
    zlib1g-dev \
    libharfbuzz-dev \
    libgtk-4-dev \
    libwebkit2gtk-4.1-dev \
    libsoup-3.0-dev

rm -rf /var/lib/apt/lists/*
EOF

# Install sccache for compilation caching
RUN cargo install sccache --locked
ENV RUSTC_WRAPPER=sccache

# Build dependencies and application
COPY --from=planner /app/recipe.json recipe.json
COPY . .
RUN <<EOF
. /etc/environment
cargo chef cook --release --target ${RUST_TARGET} --recipe-path recipe.json
cargo build --release --target ${RUST_TARGET}
cp target/${RUST_TARGET}/release/pano /app/pano
EOF

# Final stage - minimal runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libgtk-4-1 \
    libwebkit2gtk-4.1-0 \
    libsoup-3.0-0 \
    ca-certificates \
    xvfb \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/pano /usr/local/bin/pano
ENTRYPOINT ["xvfb-run", "-a", "pano", "--url", "https://google.com"]
