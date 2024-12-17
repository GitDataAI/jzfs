FROM ubuntu
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata openssl git\
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
<<<<<<< HEAD:Dockerfile
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
=======
COPY  target/release/SSHServer ./
COPY config.yaml .
COPY ed25519 .
<<<<<<< HEAD
EXPOSE 2222
CMD ["/app/SSHServer"]
>>>>>>> 249b375 (part):docker/SSHServer/Dockerfile
=======
EXPOSE 22
CMD ["/app/SSHServer"]
>>>>>>> 9be78dc (fix port 2222 -> 22)
