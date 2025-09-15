# Multi-platform Dockerfile based on Alpine
FROM --platform=$BUILDPLATFORM rust:alpine AS builder
WORKDIR /app

# Build arguments for cross-compilation
ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH

# Set Rust target based on platform
RUN case "${TARGETARCH}" in \
    "amd64") echo "x86_64-unknown-linux-musl" > /tmp/rust_target ;; \
    "arm64") echo "aarch64-unknown-linux-musl" > /tmp/rust_target ;; \
    *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    RUST_TARGET=$(cat /tmp/rust_target) && \
    echo "Building for ${RUST_TARGET}" && \
    rustup target add ${RUST_TARGET}

# Install build dependencies for Alpine
RUN apk add --no-cache \
    build-base \
    pkgconfig \
    ca-certificates \
    musl-dev \
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

# Install cross-compilation tools if needed
RUN if [ "${TARGETARCH}" = "arm64" ] && [ "${BUILDPLATFORM}" != "linux/arm64" ]; then \
        apk add --no-cache gcc-aarch64-none-elf; \
    fi

# Cache dependencies
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && \
    RUST_TARGET=$(cat /tmp/rust_target) && \
    cargo build --target ${RUST_TARGET} --release && \
    rm -rf src target/${RUST_TARGET}/release/deps/*

# Build application
COPY . .
RUN RUST_TARGET=$(cat /tmp/rust_target) && \
    cargo build --target ${RUST_TARGET} --release && \
    cp target/${RUST_TARGET}/release/pano /app/pano

# Runtime stage
FROM alpine:latest
WORKDIR /app

# Install runtime dependencies and X11/GUI packages
RUN apk add --no-cache \
    glib \
    cairo \
    pango \
    gdk-pixbuf \
    libatk-1.0 \
    fontconfig \
    freetype \
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
    dbus \
    dbus-x11 \
    mesa-gl \
    mesa-egl \
    mesa-gles \
    mesa-dri-gallium \
    mesa-va-gallium \
    mesa-gbm \
    ttf-freefont \
    font-noto \
    xvfb \
    adwaita-icon-theme \
    hicolor-icon-theme \
    shared-mime-info \
    zlib \
    harfbuzz \
    gettext \
    gtk4.0 \
    webkit2gtk-4.1 \
    libsoup3

# Copy binary from builder
COPY --from=builder /app/pano /usr/local/bin/pano

# Set up NVIDIA GPU acceleration environment
ENV LIBGL_ALWAYS_INDIRECT=0 \
    __GLX_VENDOR_LIBRARY_NAME=nvidia \
    NVIDIA_DRIVER_CAPABILITIES=all \
    NVIDIA_VISIBLE_DEVICES=all

# Initialize D-Bus and create machine-id
RUN dbus-uuidgen > /etc/machine-id

