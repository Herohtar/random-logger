FROM rust:1.61.0-alpine3.15 as build

COPY . ./src/random-logger

WORKDIR /src/random-logger

ENV RUSTFLAGS="-C target-feature=+crt-static"

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=build /src/random-logger/target/x86_64-unknown-linux-musl/release/random-logger .

ENTRYPOINT [ "/random-logger" ]
