FROM ubuntu:22.04
WORKDIR /tmps
COPY . .
RUN ls .
WORKDIR /app
RUN cp /tmps/target/release/GitDataOS .
RUN chmod +x GitDataOS
RUN rm -rf /tmps
CMD ["/app/GitDataOS"]
