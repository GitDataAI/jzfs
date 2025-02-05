FROM ubuntu:22.04
WORKDIR /app
COPY target/release/gitdata-user .
ENTRYPOINT ["./gitdata-user"]
EXPOSE 80
