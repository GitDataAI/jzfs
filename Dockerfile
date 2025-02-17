FROM ubuntu:22.04
WORKDIR /app
COPY target/release/GitDataOS .
RUN chmod +x GitDataOS
CMD ["/app/GitDataOS"]
