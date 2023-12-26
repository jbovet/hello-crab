FROM rust:1.74-slim

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["hello-crab"]