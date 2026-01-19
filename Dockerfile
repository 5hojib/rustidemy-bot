# Stage 1: Build the frontend
FROM node:20-slim AS frontend-builder
WORKDIR /app
COPY app/package*.json ./
RUN npm install
COPY app/ .
RUN npm run build

# Stage 2: Build the backend
FROM 5hojib/rustidemy:latest AS backend-builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 3: Create the final image
FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Create a non-root user
RUN useradd -m appuser
USER appuser

# Copy the built binary from the backend builder
COPY --from=backend-builder /app/target/release/rustidemy-bot .

# Copy the built frontend assets from the frontend builder
COPY --from=frontend-builder /app/dist ./app/dist

CMD ["./rustidemy-bot"]
