# docker build -t rust-server:v1 .

FROM rust:1.41.1-slim

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["main"]

# docker run -it --rm --name rust-server -p 8080:5000 rust-server:v1