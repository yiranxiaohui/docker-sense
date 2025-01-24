FROM web.yunnet.top:12500/ubuntu:latest
LABEL authors="root"

RUN mkdir -p "/opt/docker"

COPY DockSense /opt/DockSense
COPY config.toml /opt/DockSense

WORKDIR /opt/DockSense

RUN chmod +x /opt/docker/DockSense

CMD ["./DockSense"]
