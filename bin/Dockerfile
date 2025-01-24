FROM web.yunnet.top:12500/ubuntu:latest
LABEL authors="root"

RUN mkdir -p "/opt/docker"

COPY witdata-docker /opt/docker
COPY config.toml /opt/docker

WORKDIR /opt/docker

RUN chmod +x /opt/docker/witdata-docker

CMD ["./witdata-docker"]
