FROM ubuntu:22.04
WORKDIR /app
COPY /target/release/gitdata-auth .
ENTRYPOINT ["./gitdata-auth"]
EXPOSE 80
