# Build stage
FROM rust:1.87 AS chef
USER root
RUN apt-get update && apt-get install -y pkg-config libssl-dev nodejs npm
RUN cargo install cargo-chef --locked
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app

# Add build argument
ARG ROT_API_URL
ENV ROT_API_URL=$ROT_API_URL

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Ensure ROT_API_URL is available in this stage
ARG ROT_API_URL
ENV ROT_API_URL=$ROT_API_URL
RUN echo "Building with ROT_API_URL: $ROT_API_URL"
RUN cargo build --release
RUN npm install
RUN npx tailwindcss -i ./src/style.css -o ./dist/style.css
RUN trunk build --release

# Runtime stage with nginx
FROM nginx:alpine AS runtime
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
