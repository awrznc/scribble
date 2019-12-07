FROM php:7.3-fpm

COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

RUN apt-get upgrade && \
    apt-get update
RUN apt-get install -y git

WORKDIR /root/src
