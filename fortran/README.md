Fortran
===

数値計算に適したプログラミング言語。

## 環境構築

```bash
docker build -t local_fortran:2018 .
docker run --rm -it -v "$(pwd):/opt/workspace" -w /opt/workspace local_fortran:2018 bash
source /opt/intel/oneapi/setvars.sh
```

## Run

```bash
ifort ./src/hello_world/main.f90 -o ./src/hello_world/main && ./src/hello_world/main
```
