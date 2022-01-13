FROM debian:11

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y git wget make libncurses-dev flex bison gperf python3 python3-serial python3-pip curl

RUN git clone https://github.com/syndbg/goenv.git ~/.goenv \
    && ~/.goenv/bin/goenv install 1.17.6 \
    && ~/.goenv/bin/goenv global 1.17.6
ENV GOENV_ROOT="~/.goenv"
ENV PATH="${GOENV_ROOT}/bin:${PATH}"
RUN echo 'eval "$(goenv init -)"' >> ~/.bashrc

RUN mkdir -p /root/esp
WORKDIR /root/esp
RUN curl -LO "https://dl.espressif.com/dl/xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz" \
    && tar -xzf xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz
ENV PATH=$PATH:/root/esp/xtensa-esp32-elf/bin
RUN curl -LO "https://github.com/tinygo-org/tinygo/releases/download/v0.21.0/tinygo_0.21.0_amd64.deb" \
    && dpkg -i tinygo_0.21.0_amd64.deb
ENV PATH=$PATH:/usr/local/tinygo/bin
RUN apt-get install -y gcc-avr avr-libc avrdude clang-11 llvm-11-dev lld-11 libclang-11-dev
RUN /root/.goenv/shims/go get -d -u github.com/tinygo-org/tinygo
