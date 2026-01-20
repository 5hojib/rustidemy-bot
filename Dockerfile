# Frontend build stage
FROM node:20-slim AS frontend
WORKDIR /app
COPY app/package*.json ./
RUN npm install
COPY app/ .
RUN npm run build

# Backend build stage
FROM rust:1.78-slim AS backend
WORKDIR /app
COPY backend/ .
RUN cargo build --release

# Final image
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=frontend /app/dist ./app/dist
COPY --from=backend /app/target/release/backend .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 3000
CMD ["./backend"]
