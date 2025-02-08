FROM ubuntu:22.04
WORKDIR /app
COPY target/release/gitdata-auth .
RUN chmod +x gitdata-auth
ENTRYPOINT ["./gitdata-auth"]
EXPOSE 80
