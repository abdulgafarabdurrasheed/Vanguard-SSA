FROM node:22-bookworm-slim AS ui-builder
WORKDIR /ui
COPY vanguard-ui/package.json vanguard-ui/package-lock.json ./
RUN npm ci
COPY vanguard-ui ./
ARG VITE_BACKEND_URL
ENV VITE_BACKEND_URL=${VITE_BACKEND_URL}
RUN npm run build

FROM rust:1-slim-bookworm AS rust-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home app

WORKDIR /app
COPY --from=rust-builder /app/target/release/Vanguard-SSA ./Vanguard-SSA
COPY --from=ui-builder /ui/dist ./public
COPY --chown=app:app active_satellites.txt debris.txt ./
USER app
EXPOSE 3000
CMD ["./Vanguard-SSA"]
