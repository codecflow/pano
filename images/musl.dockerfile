# Multi-architecture Dockerfile for musl-based builds
FROM --platform=$BUILDPLATFORM rust:alpine AS chef
# Install build dependencies needed for cargo-chef
RUN apk add --no-cache build-base musl-dev gcc
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Determine RUST_TARGET based on TARGETPLATFORM and set as build arg
ARG RUST_TARGET
RUN case "${TARGETPLATFORM}" in \
    "linux/amd64") echo "x86_64-unknown-linux-musl" > /tmp/target ;; \
    "linux/arm64") echo "aarch64-unknown-linux-musl" > /tmp/target ;; \
    *) echo "Unsupported platform: ${TARGETPLATFORM}" && exit 1 ;; \
    esac

# Set RUST_TARGET as environment variable for the build
ENV RUST_TARGET=""
RUN RUST_TARGET=$(cat /tmp/target) && \
    echo "Building for ${RUST_TARGET}" && \
    rustup target add ${RUST_TARGET} && \
    if [ "${RUST_TARGET}" = "aarch64-unknown-linux-musl" ]; then \
        apk add --no-cache gcc-aarch64-none-linux-gnu; \
    fi

# Install GUI dependencies and OpenSSL for sccache
RUN <<EOF
apk add --no-cache \
    build-base \
    pkgconfig \
    ca-certificates \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    glib-dev \
    cairo-dev \
    pango-dev \
    gdk-pixbuf-dev \
    libatk-1.0 \
    fontconfig-dev \
    freetype-dev \
    libx11-dev \
    libxext-dev \
    libxrender-dev \
    libxi-dev \
    libxrandr-dev \
    libxcursor-dev \
    libxcomposite-dev \
    libxdamage-dev \
    libxfixes-dev \
    libxscrnsaver-dev \
    zlib-dev \
    harfbuzz-dev \
    gettext-dev \
    gtk4.0-dev \
    webkit2gtk-4.1-dev \
    libsoup3-dev
EOF

# Install sccache for compilation caching
RUN cargo install sccache --locked
ENV RUSTC_WRAPPER=sccache

# Build dependencies and application
COPY --from=planner /app/recipe.json recipe.json
COPY . .
RUN RUST_TARGET=$(cat /tmp/target) && \
    echo "Building with target: ${RUST_TARGET}" && \
    cargo chef cook --release --target ${RUST_TARGET} --recipe-path recipe.json && \
    cargo build --release --target ${RUST_TARGET} && \
    cp target/${RUST_TARGET}/release/pano /app/pano

# Final stage - minimal runtime image
FROM alpine
RUN apk add --no-cache \
    # GTK and WebKit (core application dependencies)
    gtk4.0 \
    webkit2gtk-4.1 \
    libsoup3 \
    ca-certificates \
    # Core GTK dependencies
    glib \
    cairo \
    pango \
    gdk-pixbuf \
    libatk-1.0 \
    # X11 libraries
    libx11 \
    libxext \
    libxrender \
    libxi \
    libxrandr \
    libxcursor \
    libxcomposite \
    libxdamage \
    libxfixes \
    libxscrnsaver \
    # Mesa for software rendering
    mesa-gl \
    mesa-egl \
    mesa-gles \
    mesa-dri-gallium \
    mesa-gbm \
    # Font rendering
    fontconfig \
    freetype \
    # Additional libraries
    zlib \
    harfbuzz \
    gettext \
    # Virtual framebuffer
    xvfb \
    # Essential for GTK communication
    dbus

# Initialize D-Bus machine ID
RUN dbus-uuidgen > /etc/machine-id

COPY --from=builder /app/pano /usr/local/bin/pano
ENTRYPOINT ["xvfb-run", "-a", "pano", "--url", "https://google.com"]
