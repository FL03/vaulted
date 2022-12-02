FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --workspace

FROM photon as app-base

RUN yum update -y && yum upgrade -y

FROM app-base as runner

COPY --from=builder /project/target/release/vaulted-cli /bin/vaulted-cli

ENTRYPOINT ["vaulted-cli"]
