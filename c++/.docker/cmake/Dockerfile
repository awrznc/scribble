FROM debian:10

RUN apt update && apt upgrade -y
RUN apt install -y gcc g++ curl make ninja-build

ENV INSTALL_DIR "/usr/local/"
WORKDIR /tmp
RUN curl -OL https://github.com/Kitware/CMake/releases/download/v3.19.0/cmake-3.19.0-Linux-x86_64.sh && \
    sh cmake-3.19.0-Linux-x86_64.sh -- --skip-license --prefix=$INSTALL_DIR && \
    rm -rf cmake-3.19.0-Linux-x86_64.sh

WORKDIR /home/src
