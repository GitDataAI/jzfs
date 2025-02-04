FROM ubuntu:22.04
WORKDIR /app
COPY target/release/gateway .
ENTRYPOINT ["./gateway"]
EXPOSE 80
