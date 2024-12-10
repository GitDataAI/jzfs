FROM ubuntu
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata openssl git\
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY  target/release/ControlPlane ./
RUN mkdir "config"
COPY config/config.toml ./config/
<<<<<<< HEAD
EXPOSE 80
=======

<<<<<<< HEAD
EXPOSE 80, 2222
>>>>>>> 8084810 (:label: part)
=======
EXPOSE 80
EXPOSE 2222
>>>>>>> 0db9e89 (:whale: fix expose port)
CMD ["/app/ControlPlane"]