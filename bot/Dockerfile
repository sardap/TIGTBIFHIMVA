FROM rust:1.56.1-slim as rust_builder

RUN apt-get -y update && apt-get install -y pkg-config libssl-dev

RUN USER=root cargo new --bin bot
WORKDIR /bot
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock

RUN rm ./target/release/deps/bot*
RUN cargo build --release

FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata 

EXPOSE 3030

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=rust_builder /bot/target/release/bot ${APP}/bot

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./bot"]
