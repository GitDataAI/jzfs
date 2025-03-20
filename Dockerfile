FROM ubuntu:22.04
WORKDIR /app
COPY target/release/jzfs .
RUN apt-get update && apt-get install -y ca-certificates git libssl-dev openssl && update-ca-certificates
RUN chmod +x GitDataOS
CMD ["/app/jzfs"]
