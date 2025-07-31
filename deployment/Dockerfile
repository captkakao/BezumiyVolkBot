FROM rust:1.87.0 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app && chmod 755 /app

COPY --from=builder /app/target/release/BezumniyVolkBot /usr/local/bin/bot
RUN chmod +x /usr/local/bin/bot

COPY --from=builder /app/dictionaries.json /app/dictionaries.json

RUN useradd -r -s /bin/false -d /app botuser
RUN chown -R botuser:botuser /app

USER botuser
WORKDIR /app

CMD ["bot"]