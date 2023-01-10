FROM rust:1.66.0

WORKDIR /usr/src/echbot
COPY . .

RUN cargo install --path .

cmd "echbot"
