FROM rust:latest

RUN rustup install 1.38.0
RUN rustup default 1.38.0
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update && apt-get install -y make build-essential musl-tools gnuplot

WORKDIR build
COPY . .

RUN cargo build --tests --target=x86_64-unknown-linux-musl
RUN cargo build --benches --target=x86_64-unknown-linux-musl --release
CMD ["cargo", "test", "--target=x86_64-unknown-linux-musl"]
