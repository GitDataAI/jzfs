FROM ubuntu
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata openssl\
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY  target/release/ControlPlane ./
RUN mkdir "config"
COPY config/config.toml ./config/
EXPOSE 80
CMD ["/app/ControlPlane"]