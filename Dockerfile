FROM ubuntu:22.04
WORKDIR /tmps
COPY . .
WORKDIR /app
RUN cp /tmps/target/release/GitDataOS .
RUN chmod +x GitDataOS
RUN rm -rf /tmps
CMD ["/app/GitDataOS"]
