FROM ubuntu:22.04


WORKDIR /app

COPY jzfs /jzfs
COPY script/start.sh /start.sh

RUN chmod +x /start.sh

ENTRYPOINT ["/start.sh"]
