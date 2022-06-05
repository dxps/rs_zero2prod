## -------------------------------------------
##  The cargo-chef recipe Stage
## -------------------------------------------

FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 as chef

WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file
RUN cargo chef prepare --recipe-path recipe.json

## -------------------------------------------
##  The Dependencies Build Stage
## -------------------------------------------

FROM chef as build
COPY --from=planner /app/recipe.json recipe.json
# Build the project dependencies, not the application!
RUN cargo chef cook --release --recipe-path recipe.json

## -------------------------------------------
##  The Application Build Stage
## -------------------------------------------

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
