FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release

FROM photon as app-base

RUN yum update -y && yum upgrade -y

FROM app-base as latest

COPY --from=builder /project/target/release/vaulted /bin/vaulted

CMD ["vaulted"]
