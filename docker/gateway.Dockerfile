FROM ubuntu:22.04
WORKDIR /app
COPY target/release/gateway .
RUN chmod +x ./gateway
ENTRYPOINT ["./gateway"]
EXPOSE 80
