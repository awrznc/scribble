MessagePack
===

## setup

```bash
# dnf install -y gcc-c++ git make cmake ruby ruby-devel rpm-build 
# gem install bundler
git clone https://github.com/msgpack/msgpack-c.git --depth 1 --recursive -b cpp-3.3.0
bundle config set path "vendor/bundle" && bundle install
```

## build

```bash
mkdir -p ./build && cd ./build && cmake -DMSGPACK_BUILD_EXAMPLES=OFF .. && make && ./main_app
```
