FROM centos:latest

RUN yum update -y &&\
    yum upgrade -y &&\
    yum clean all

# gcc
WORKDIR /usr/local/src
RUN yum install -y gcc gcc-c++ bzip2 make
RUN curl -LO http://ftp.jaist.ac.jp/pub/GNU/gcc/gcc-9.1.0/gcc-9.1.0.tar.gz && \
    tar zxvf gcc-9.1.0.tar.gz
WORKDIR /usr/local/src/gcc-9.1.0
RUN ./contrib/download_prerequisites && \
    ./configure \
      --enable-languages=c,c++ \
      --prefix=/usr/local/lib/gcc-9.1.0 \
      --disable-bootstrap \
      --disable-multilib
RUN make -j2 && make install
RUN yum remove -y gcc gcc-c++
ENV PATH $PATH:/usr/local/lib/gcc-9.1.0/bin/
RUN cp /usr/local/lib/gcc-9.1.0/lib64/libstdc++.so.6.0.26 /usr/lib64 && \
    mv /usr/lib64/libstdc++.so.6 /usr/lib64/libstdc++.so.6.bak && \
    ln -s /usr/lib64/libstdc++.so.6.0.26 /usr/lib64/libstdc++.so.6

# cmake
WORKDIR /usr/local/src
RUN curl -O https://cmake.org/files/v3.15/cmake-3.15.2.tar.gz && \
    tar zxvf cmake-3.15.2.tar.gz
WORKDIR /usr/local/src/cmake-3.15.2
RUN ./bootstrap --prefix=/usr/local/lib/cmake-3.15.2 --no-system-libs
RUN make -j2 && make install
ENV PATH $PATH:/usr/local/lib/cmake-3.15.2/bin

WORKDIR /root/src
