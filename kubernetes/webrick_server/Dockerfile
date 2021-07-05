FROM ruby:2.7

RUN gem install bundler
COPY ./src/ruby /home/src

WORKDIR /home/src/
RUN bundle install

CMD ["ruby", "main.rb"]
