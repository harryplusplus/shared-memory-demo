FROM mcr.microsoft.com/devcontainers/base:debian-12

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-c"]

RUN apt update && \
    apt install -y \
    curl \
    git \
    build-essential \
    gdb \
    lldb \
    cmake \
    # To open in a browser on the host
    xdg-utils \
    && apt clean && \
    rm -rf /var/lib/apt/lists/* && \
    ln -s /usr/bin/lldb-vscode-14 /usr/bin/lldb-dap

USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.88.0
RUN echo 'alias ll="ls -a -p -lh"' >> ~/.bashrc

WORKDIR /work
