FROM rust:1.60 as build

RUN USER=root cargo new --bin spite-site
WORKDIR /spite-site

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/spite-site*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /meldback/target/release/spite-site .
CMD ["./spite-site"]