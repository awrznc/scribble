[BITS 16]

org     0x7c00
jmp     entry
db      0x90
db      "SMPLIPL "
dw      512
db      1
dw      1
db      2
dw      224
dw      2880
db      0xf0
dw      9
dw      18
dw      2
dd      0
dd      2880
db      0, 0, 0x29
dd      0xffffffff
db      "SAMPLE-OS  "
db      "FAT12   "
times   18  db 0x00

entry:
    mov ax,0
    mov ss,ax
    mov sp,0x7c00
    mov ds,ax
    mov es,ax

    ; read init
    mov ax,0x0820
    mov es,ax
    mov ch,0
    mov dh,0
    mov cl,2

readloop:
    mov si,0

read_point:
    mov ah,0x02
    mov al,1
    mov bx,0
    mov dl,0x00
    int 0x13

    ; read check
    jnc next
    add si,1
    cmp si,5
    jae error

    ; reset
    mov ah,0x00
    mov dl,0x00
    int 0x13
    jmp read_point

next:
    mov ax,es
    add ax,0x0020
    mov es,ax
    add cl,1
    cmp cl,18
    jbe readloop

error:
    mov si,message

putloop:
    mov al,byte [ds:si]
    add si,1
    cmp al,0
    je  fin
    mov ah,0x0e
    mov bx,15
    int 0x10
    jmp putloop

fin:
    hlt
    jmp fin

message:
    db  "finished read sector."
    db  0

times   0x1fe-($-$$)    db  0x00

db      0x55, 0xaa

db      0xf0, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00
times   4600    db  0x00
db      0xf0, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00
times   1469432 db  0x00
