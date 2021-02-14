FROM rust:latest

WORKDIR /rs9cc
COPY src ./src
COPY Cargo.* ./
RUN cargo build
COPY . .
ENTRYPOINT [ "./bin/test.sh" ]