FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/rusticon/releases/latest/download/rusticon-terminal-linux.zip \
    -o /tmp/rusticon.zip \
    && unzip -o /tmp/rusticon.zip -d /usr/local/bin \
    && rm /tmp/rusticon.zip \
    && chmod +x /usr/local/bin/rusticon

CMD ["/bin/bash", "-i"]
