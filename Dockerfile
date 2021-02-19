FROM rust:latest

WORKDIR /rs9cc
COPY Cargo.* ./
COPY src ./src
RUN cargo build
COPY . .
ENTRYPOINT [ "./bin/test.sh" ]