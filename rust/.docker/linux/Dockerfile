FROM centos:centos8

# install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH $PATH:$HOME/.cargo/bin/

RUN dnf upgrade -y && \
    dnf install -y gcc

# setting for cross-compile to windows
RUN dnf --enablerepo=PowerTools install -y mingw64-gcc mingw64-winpthreads-static
RUN $HOME/.cargo/bin/rustup target add x86_64-pc-windows-gnu && \
    echo -n -e "[target.x86_64-pc-windows-gnu]\nlinker = \"/usr/bin/x86_64-w64-mingw32-gcc\"" >> ~/.cargo/config

# install sdl2
## centos
RUN dnf --enablerepo=PowerTools install -y SDL2-devel
## windows
WORKDIR /tmp
RUN curl -OL https://www.libsdl.org/release/SDL2-devel-2.0.10-mingw.tar.gz && \
    tar zxvf SDL2-devel-2.0.10-mingw.tar.gz && \
    cp -r SDL2-2.0.10/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/

# open-ssl-devel
RUN dnf install -y openssl-devel

WORKDIR /home/src
