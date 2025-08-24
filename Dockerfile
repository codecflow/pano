FROM rust:1-bookworm AS builder
ARG GTK_VARIANT=gtk4
WORKDIR /app

# System deps
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential pkg-config ca-certificates \
    libgdk-pixbuf2.0-dev libcairo2-dev libpango1.0-dev libatk1.0-dev \
    && rm -rf /var/lib/apt/lists/*

# GTK / WebKit deps
RUN set -eux; \
    if [ "$GTK_VARIANT" = "gtk3" ]; then \
    apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    libgtk-3-dev libwebkit2gtk-4.0-dev libsoup2.4-dev \
    && rm -rf /var/lib/apt/lists/*; \
    else \
    apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    libgtk-4-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev \
    && rm -rf /var/lib/apt/lists/*; \
    fi

# Cache crates
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/*

# Build app
COPY . .
RUN cargo build --release --locked \
    && strip target/release/pano || true

# Default CMD just runs the binary (optional)
CMD ["./target/release/pano"]
