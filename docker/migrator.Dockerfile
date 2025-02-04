FROM ubuntu:22.04
WORKDIR /app
COPY target/release/migrator .
RUN chmod +x /app/migrator
CMD ["sh", "-c", "./migrator && tail -f /dev/null"]
