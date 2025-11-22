FROM rust:1.91-alpine AS builder

# CORRECCIÓN: Paquetes actualizados para Alpine Linux (apk)
# Se eliminaron los nombres estilo Debian (libfontconfig1-dev, libxcb-xfixes0-dev)
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    fontconfig-dev \
    freetype-dev \
    cmake \
    make \
    g++ \
    libxcb-dev \
    libxkbcommon-dev \
    python3

WORKDIR /app
COPY . .

# Establecer RUSTFLAGS para que el enlazador encuentre las librerías
ENV RUSTFLAGS="-L /usr/lib -L /usr/local/lib"

# Compilación optimizada
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

# Copiar solo el binario resultante
COPY --from=builder /app/target/release/parseit .

ENTRYPOINT ["./parseit"]
