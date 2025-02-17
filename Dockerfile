FROM ubuntu:22.04
WORKDIR /app
COPY target/release/GitDataOS .
RUN chmod +x GitDataOS
RUN rm -rf /tmps
CMD ["/app/GitDataOS"]
