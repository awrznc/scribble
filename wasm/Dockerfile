FROM centos:latest

# install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH $PATH:$HOME/.cargo/bin/

# install wasm-pack
RUN yum install -y gcc openssl-devel
RUN $HOME/.cargo/bin/cargo install wasm-pack && \
    $HOME/.cargo/bin/cargo install cargo-generate
RUN $HOME/.cargo/bin/rustup target add wasm32-unknown-unknown

# install npm
RUN curl -sL https://rpm.nodesource.com/setup_13.x | bash -
RUN yum install -y nodejs

# git
RUN yum -y install https://centos7.iuscommunity.org/ius-release.rpm && \
    yum -y install git2u

WORKDIR /root/src
