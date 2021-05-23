FROM rust as builder
 
WORKDIR /opt/conway/app

COPY src/* ./src/
COPY Cargo* ./

RUN cargo build --release

FROM ubuntu

WORKDIR /opt/conway/game

COPY --from=builder /opt/conway/app/target/release/gameoflife .

CMD ["./gameoflife"]