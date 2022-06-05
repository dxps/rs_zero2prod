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

FROM rust:1.61.0-slim

WORKDIR /app
# Copy the compiled binary from the build stage to this runtime stage.
COPY --from=build /app/target/release/ztp-rs ztp-rs
# Configuration is also needed.
COPY config config
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./ztp-rs" ]
