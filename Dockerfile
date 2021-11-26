FROM node:10 as frontend_builder

WORKDIR /app

COPY ./frontend/package*.json ./

RUN npm install .

COPY ./frontend .

RUN npm run build

FROM rust:1.56.1-slim as rust_builder

RUN USER=root cargo new --bin tgtb
WORKDIR /tgtb
COPY ./tgtb/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./tgtb/src ./src
COPY ./tgtb/Cargo.lock ./Cargo.lock

RUN rm ./target/release/deps/tgtb*
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

COPY --from=rust_builder /tgtb/target/release/tgtb ${APP}/tgtb
COPY --from=frontend_builder /app/dist ${APP}/website

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENV RUST_BACKTRACE=1

CMD ["./tgtb", "--web-dir", "./website"]
