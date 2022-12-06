FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN rustup update

FROM builder-base as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --workspace

FROM photon as runner-base

ENV RUST_LOG="info" \
    SERVER_PORT=8080

RUN yum update -y && yum upgrade -y

EXPOSE 80
EXPOSE 6379

FROM runner-base as runner

COPY --chown=55 Vaulted.toml /config/Vaulted.toml
COPY --from=builder /project/target/release/vaulted /bin/vaulted

VOLUME [ "/config" ]

FROM runner as cli

ENTRYPOINT [ "vaulted" ]
CMD [ "-h" ]