FROM rust:1.67

EXPOSE 8080

WORKDIR /usr/src/persona5r_calculator

COPY . .

RUN cargo install --path .

CMD ["persona5r_calculator"]