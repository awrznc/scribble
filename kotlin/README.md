Kotlin
===

## Build

```bash
docker build ./.docker/pure.Dockerfile -t kotlin:pure
docker run --rm -it -v "$(pwd):/root/work/" -w "/root/work/" kotlin:pure bash
kotlinc-native ./src/HelloWorld/Main.kt && ./program.kexe
// => Hello, World!
exit
```
