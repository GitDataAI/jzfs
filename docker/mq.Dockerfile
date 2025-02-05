FROM ubuntu:22.04
RUN apt update \
  &&  apt install ca-certificates  -y \
  && update-ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY target/release/gitdata-mq .
RUN chmod +x /app/gitdata-mq
CMD ["sh", "-c", "./gitdata-mq"]
