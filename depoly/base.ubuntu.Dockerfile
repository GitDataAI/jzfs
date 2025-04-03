FROM ubuntu:22.04
RUN apt-get update
RUN apt-get install -y ca-certificates git libssl-dev openssl
RUN update-ca-certificates
