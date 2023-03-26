
## Install

```bash
docker run --rm -it -v "$(pwd):/root/workspace" debian:10 bash
apt update -y && apt install -y curl xz-utils
cd /tmp && curl -OL 'https://ziglang.org/download/0.10.1/zig-linux-x86_64-0.10.1.tar.xz'
tar Jxfv ./zig-linux-x86_64-0.10.1.tar.xz
export PATH="${PATH}:$(pwd)/zig-linux-x86_64-0.10.1/"

# check
zig version
# => 0.10.1
```

## Run

```bash
zig run ./src/hello_world/main.zig

# or

zig build-exe ./src/hello_world/main.zig && time ./main
```
