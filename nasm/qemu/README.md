## Dev

```bash
# Build
nasm -o ./boot.img ./src/boot.asm

# 仮想ハードディスクの作成
# qemu-img.exe create -f qcow2 harddisk.qcow2 1024K

# Run
qemu-system-x86_64.exe -drive format=raw,file=boot.img

qemu-system-i386 -drive file=boot.img,format=raw,if=floppy
```
