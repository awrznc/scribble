Prolog
===

## Documentation

https://www.swi-prolog.org/

## Example

```bash
$ docker build -t prolog:latest -f ./Dockerfile .
$ docker run --rm -it -v "$(PWD):/opt" prolog:latest bash

$ prolog --version
SWI-Prolog version 8.0.2 for x86_64-linux

# Run
$ prolog -s ./src/hello_world/main.pl
```
