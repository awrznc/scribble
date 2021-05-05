FROM centos:8

RUN dnf upgrade -y && \
    dnf install -y git gcc gcc-c++ cmake python27 && \
    dnf --enablerepo=PowerTools install -y ninja-build

# zapcc
RUN git clone --depth 1 https://github.com/yrnkrn/zapcc.git /usr/local/zapcc && \
    mkdir /usr/local/zapcc/build
WORKDIR /usr/local/zapcc/build
RUN cmake -G Ninja -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_WARNINGS=OFF ..
RUN ninja -j3
ENV PATH "$PATH:/usr/local/zapcc/build/bin"

WORKDIR /home/src
