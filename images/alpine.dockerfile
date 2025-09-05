FROM rust:alpine AS builder
ARG GTK_VARIANT=gtk4
WORKDIR /app

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
    gettext-dev

# Install GTK/WebKit dependencies based on variant
RUN set -eux; \
    if [ "$GTK_VARIANT" = "gtk3" ]; then \
        apk add --no-cache \
            gtk+3.0-dev \
            webkit2gtk-dev \
            libsoup-dev; \
    else \
        apk add --no-cache \
            gtk4.0-dev \
            webkit2gtk-4.1-dev \
            libsoup3-dev; \
    fi

# Cache dependencies
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs \
    && cargo build \
    && rm -rf src target/debug/deps/*

# Build application
COPY . .
# Set environment to use dynamic linking instead of static
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release --locked

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
    gettext

# Install debugging tools
RUN apk add --no-cache \
    strace \
    gdb \
    file \
    musl-utils

# Install GTK/WebKit runtime libraries
ARG GTK_VARIANT=gtk4
RUN set -eux; \
    if [ "$GTK_VARIANT" = "gtk3" ]; then \
        apk add --no-cache \
            gtk+3.0 \
            webkit2gtk \
            libsoup; \
    else \
        apk add --no-cache \
            gtk4.0 \
            webkit2gtk-4.1 \
            libsoup3; \
    fi

# Copy binary from builder
COPY --from=builder /app/target/release/pano /usr/local/bin/pano

# Set up environment for debugging and GUI
ENV GTK_DEBUG=all \
    G_MESSAGES_DEBUG=all \
    RUST_BACKTRACE=1 \
    RUST_LOG=debug

# Set up NVIDIA GPU acceleration environment
ENV LIBGL_ALWAYS_INDIRECT=0 \
    __GLX_VENDOR_LIBRARY_NAME=nvidia \
    NVIDIA_DRIVER_CAPABILITIES=all \
    NVIDIA_VISIBLE_DEVICES=all

# Initialize D-Bus and create machine-id
RUN dbus-uuidgen > /etc/machine-id

CMD ["pano"]
