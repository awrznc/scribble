Linker Script
===

## Run

```bash
apt update -y && apt install -y binutils

cd ./src/hello_world/

# one liner
# cat ./hello.txt ./world.txt > ./hello_world.txt
ld -T /dev/null -format binary ./hello.txt ./world.txt -o hello_world.txt

# with script
ld -T ./main.ld
```
