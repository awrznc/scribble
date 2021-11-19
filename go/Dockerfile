FROM debian:latest

RUN apt-get update -y && apt-get install -y git curl

RUN git clone https://github.com/syndbg/goenv.git ~/.goenv && \
    ~/.goenv/bin/goenv install 1.17.3

ENV GOENV_ROOT="~/.goenv"
ENV PATH="${GOENV_ROOT}/bin:${PATH}"

RUN echo 'eval "$(goenv init -)"' >> ~/.bashrc
