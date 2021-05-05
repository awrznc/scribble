FROM centos:8

RUN yum update -y && \
    yum -y install bzip2 gcc openssl-devel readline-devel zlib-devel make

RUN yum -y install git
RUN git clone https://github.com/rbenv/rbenv.git ~/.rbenv && \
    echo 'export PATH="$HOME/.rbenv/bin:$PATH"' >> ~/.bashrc && \
    echo 'eval "$(rbenv init -)"' >> ~/.bashrc
ENV PATH /root/.rbenv/shims:/root/.rbenv/bin:$PATH
RUN git clone https://github.com/rbenv/ruby-build.git ~/.rbenv/plugins/ruby-build && \
    ~/.rbenv/bin/rbenv install 2.7.1 && \
    ~/.rbenv/bin/rbenv global 2.7.1
ENV RUBYOPT -EUTF-8
RUN gem install bundler -v "2.1.4"

# rails
RUN curl --silent --location https://dl.yarnpkg.com/rpm/yarn.repo | tee /etc/yum.repos.d/yarn.repo
RUN yum -y install gcc-c++ sqlite-devel nodejs yarn
RUN gem install rails -v "6.0.3"

# PostgreSQL
RUN yum -y install postgresql-devel

WORKDIR /home
