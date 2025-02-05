FROM ubuntu:22.04
WORKDIR /app
COPY target/release/gitdata-mq .
RUN chmod +x /app/gitdata-mq
CMD ["sh", "-c", "./gitdata-mq"]
