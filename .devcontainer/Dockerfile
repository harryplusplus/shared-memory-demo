FROM mcr.microsoft.com/devcontainers/base:debian-12

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-c"]

RUN apt update && \
    apt install -y \
    curl \
    git \
    # To open in a browser on the host
    xdg-utils \
    && apt clean && \
    rm -rf /var/lib/apt/lists/*

USER vscode
ENV HOME="/home/vscode"
RUN echo 'alias ll="ls -a -p -lh"' >> ~/.bashrc

WORKDIR /work
