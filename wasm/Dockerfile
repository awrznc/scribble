FROM centos:8

# # install rust
# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# ENV PATH $PATH:$HOME/.cargo/bin/

# # install wasm-pack
# RUN yum install -y gcc openssl-devel
# RUN $HOME/.cargo/bin/cargo install wasm-pack && \
#     $HOME/.cargo/bin/cargo install cargo-generate
# RUN $HOME/.cargo/bin/rustup target add wasm32-unknown-unknown

# # install npm
# RUN curl -sL https://rpm.nodesource.com/setup_13.x | bash -
# RUN yum install -y nodejs

# # git
# RUN yum -y install https://centos7.iuscommunity.org/ius-release.rpm && \
#     yum -y install git2u

# install emcc
RUN dnf install -y git python36 bzip2 unzip
RUN ln --symbolic /usr/bin/python3 /usr/bin/python
WORKDIR /home/
RUN git clone --depth 1 https://github.com/emscripten-core/emsdk.git
WORKDIR /home/emsdk
RUN bash -x ./emsdk install latest && \
    bash -x ./emsdk activate latest
RUN echo "cd /home/emsdk && source /home/emsdk/emsdk_env.sh && cd -" >> /root/.bashrc
ENV PYTHON="python3"

# install deno
RUN curl -fsSL https://deno.land/x/install/install.sh | sh
RUN ln --symbolic /root/.deno/bin/deno /usr/bin/deno

# install node
RUN dnf install -y nodejs

WORKDIR /home/src
