FROM centos:latest

# install npm
RUN curl -sL https://rpm.nodesource.com/setup_13.x | bash -
RUN yum install -y nodejs

## install nuxtjs
RUN npm install -g vue-cli

WORKDIR /root/src
