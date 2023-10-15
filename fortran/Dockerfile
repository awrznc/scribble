# https://www.intel.com/content/www/us/en/docs/oneapi/installation-guide-linux/2024-0/apt.html#GUID-2A09AD83-B3A4-472F-857A-7C5E746087AB

FROM debian:12

ARG PKG_NAME=intel-fortran-compiler-2023.2.1
ARG GPG_FILE_PATH=/usr/share/keyrings/oneapi-archive-keyring.gpg

RUN apt-get update -y && apt-get install -y wget gnupg
RUN wget -O- https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB \
        | gpg --dearmor \
        | tee ${GPG_FILE_PATH} > /dev/null
RUN echo "deb [signed-by=${GPG_FILE_PATH}] https://apt.repos.intel.com/oneapi all main" | tee /etc/apt/sources.list.d/oneAPI.list
RUN apt-get update -y && apt install -y "${PKG_NAME}" gcc
