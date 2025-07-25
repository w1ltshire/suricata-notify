FROM rust:1.88.0-slim-bullseye AS build

# View app name in Cargo.toml
ARG APP_NAME=suricata-notify

WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Install the ca-certificate package
RUN apt-get update && apt-get install -y ca-certificates
# Update the CA certificates in the container
RUN update-ca-certificates

COPY Cargo.lock Cargo.toml ./
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

COPY src src
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final

# Install the ca-certificate package
RUN apt-get update && apt-get install -y ca-certificates
# Update the CA certificates in the container
RUN update-ca-certificates

COPY --from=build /bin/server /bin/
CMD ["/bin/server"]
