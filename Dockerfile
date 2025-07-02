FROM 5hojib/rustidemy:latest AS builder

COPY src/ src/
COPY templates/ templates/

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/bot .

CMD ["./bot"]