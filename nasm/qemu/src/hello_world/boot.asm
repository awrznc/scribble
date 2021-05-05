[BITS 16]

; db: data byte.        ファイルの内容を1バイト書く命令
; dw: data word.        ファイルの内容を2バイト書く命令
; dd: data doubel-word. ファイルの内容を4バイト書く命令

; times: 繰り返し

; org: origin. 機械語が実行時にどこに読み込まれるのか指定する命令
; $: 読み込まれる予定のメモリの番地

; []: memory.

; jmp: jamp. 指定した場所に移動する命令
; mov: move. 代入命令
; cmp: compare. 比較命令
; je:  条件ジャンプ命令
; int: interrupt. ソフトウェア割込み命令(BIOSの機能が利用できる)
; hlt: halt. キーボードやマウスなどの外部機器が動くまでCPUを停止する

; === 16bit register ===
; ax: accumulator extend. アキュムレータ。累積演算器
; cx: counter extend. カウンタ。数を数える機械
; dx: data extend. データ
; bx: base extend. 基点（土台）
; sp: stack pointer. スタック用ポインタ
; bp: base pointer. ベース用ポインタ
; si: source index. 読み込みインデックス
; di: destination index. 書き込みインデックス
; extendはhighとlowの8bit registerで構成されている。
; 32bitに拡張する場合は、先頭にextendの`e`(例: eax)を付ける。
; 64bitに拡張する場合は、先頭に`r`(例: rax)を付ける。

; === segment register ===
; es: extra segment 1.
; fs: extra segment 2.
; gs: extra segment 3.
; cs: code segment.
; ss: stack segment.
; ds: data segment.

org     0x7c00
jmp     entry
db      0x90
db      "SMPLIPL "      ; ブートセクタの名前 (自由) (8 Byte)
dw      512             ; 1セクタの大きさ (512固定)
db      1               ; クラスタの大きさ (1セクタ固定)
dw      1               ; FATがどこから始まるか (普通は1セクタ目からにする)
db      2               ; FATの個数 (2固定)
dw      224             ; ルートディレクトリ領域の大きさ (普通は224エントリにする)
dw      2880            ; このドライブの大きさ (2880セクタ固定)
db      0xf0            ; メディアの種類 (0xf0固定)
dw      9               ; FAT領域の長さ (9セクタ固定)
dw      18              ; 1トラックにいくつのセクタがあるか (18固定)
dw      2               ; ヘッドの数 (2固定)
dd      0               ; パーティションを使っていないのでここは必ず0
dd      2880            ; このドライブの大きさ(再)
db      0, 0, 0x29      ; よくわからないけどこの値にしておくといいらしい
dd      0xffffffff      ; たぶんボリュームシリアル番号
db      "SAMPLE-OS  "   ; ディスクの名前 (11 Byte)
db      "FAT12   "      ; フォーマットの名前 (8 Byte)
times   18  db 0x00     ; 18バイトのバッファ

entry:
    mov ax,0    ; レジスタの初期化
    mov ss,ax
    mov sp,0x7c00
    mov ds,ax
    mov es,ax

    mov si,message

putloop:
    mov al,byte [si]
    add si,1
    cmp al,0
    je  fin     ; alが0の場合は終了する
    mov ah,0x0e
    mov bx,15
    int 0x10
    jmp putloop

fin:
    hlt         ; 何かあるまでCPUを停止させる
    jmp fin     ; 無限ループ

message:
    db  "Hello, World!"
    ; db  0x0a, 0x0a, 0x0a  ; 改行3つ
    db  0

; fill 0x00
times   0x1fe-($-$$)    db  0x00

db      0x55, 0xaa

; ブート以外の記述

db      0xf0, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00
times   4600    db  0x00
db      0xf0, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00
times   1469432 db  0x00
