FROM gitdatateam/base-ubuntu:latest
WORKDIR /app
COPY target/release/jzfs .
RUN chmod +x jzfs
CMD ["/app/jzfs"]
