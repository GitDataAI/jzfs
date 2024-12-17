FROM ubuntu
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata openssl git\
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY  target/release/ApiServer ./
COPY config.yaml .

EXPOSE 80
CMD ["/app/ApiServer"]