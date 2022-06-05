## -------------------------------------------
##  The Build Stage
## -------------------------------------------

FROM rust:1.61.0 AS build

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

## -------------------------------------------
##  The Runtime Stage
## -------------------------------------------

FROM debian:bullseye-slim AS runtime

WORKDIR /app
# Install OpenSSL (as being dynamically linked by some of our dependencies)
# and ca-certificates (as needed to verify certifs during HTTPS handshakes)
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the build stage to this runtime stage.
COPY --from=build /app/target/release/ztp-rs ztp-rs
# Configuration is also needed.
COPY config config

ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./ztp-rs" ]
