## Build

```
mkdir -p /home/src/sample/cxx/build &&  cd /home/src/sample/cxx/build && \
    rm -rf ./* && CC=gcc CXX=g++ cmake -G Ninja .. && ninja -j4 && ./server
```
