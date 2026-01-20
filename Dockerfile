# Frontend build stage
FROM node:20-slim AS frontend
WORKDIR /app
COPY app/package*.json ./
RUN npm install
COPY app/ .
RUN npm run build

# Backend build stage
FROM rust:1.82.0-slim AS backend
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY backend/ .
RUN cargo build --release

# Final image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=frontend /app/dist ./dist
COPY --from=backend /app/target/release/backend .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 3000
CMD ["./backend"]
