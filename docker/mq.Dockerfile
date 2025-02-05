FROM ubuntu:22.04
RUN apk --no-cache add ca-certificates \
  && update-ca-certificates

WORKDIR /app
COPY target/release/gitdata-mq .
RUN chmod +x /app/gitdata-mq
CMD ["sh", "-c", "./gitdata-mq"]
