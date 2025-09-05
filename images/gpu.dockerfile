FROM rust:1-bookworm
WORKDIR /app

# System deps
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential pkg-config ca-certificates \
    libgdk-pixbuf2.0-dev libcairo2-dev libpango1.0-dev libatk1.0-dev \
    libgtk-4-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev \
    && rm -rf /var/lib/apt/lists/*

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
