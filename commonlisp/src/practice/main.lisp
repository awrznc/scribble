; ==== コメント ==============================

#|  こ
    れ
    で
    も
    OK  |#


; ==== 出力 ==============================

; `~%` は改行
; Lisp の命令はリストで構成される
(format t "Hello, CommonLisp!~%")

; こちらでもOK
; (terpri) で改行できる
(print 42)(terpri)
(write 42)(terpri)


; ==== 変数 ==============================

; parametor
(defparameter parametor 110)
(write parametor)(terpri)

; special (global variable)
(defvar *special-variable*)
(setq *special-variable* 111) ; 代入
(write *special-variable*)(terpri)

; lexical (local variable)
(let ((local-variable 112))
    (write local-variable)
    (terpri))
; => 112


; 四則演算 ==============================

; 1 + 1
(+ 1 1)

; 変数に入れて表示
(let ((two (+ 1 1)))
    (write two)(terpri))
; => 2

; 2 + 3 * 3 / 2 - 0
(let ((result (- (+ 2 (/ (* 3 3) 2)) 0)))
    (write result)(terpri))
; => 13/2 (分数が返る)


; ==== 配列 ==============================

; 一次元
#(1 2 3)

; これでもOK? (要調査)
(quote (1 2 3))
'(1 2 3) ; 上記の糖衣構文

; 二次元
#2A((1 2 3) (4 5 6))

; 三次元
#3A(((1 2 3) (4 5 6))((7 8 9) (10 11 12)))


; ==== 条件分岐 ==============================
(if (= 0 1)
    (write "0 == 1")
    (write "0 != 1"))(terpri)
; => "0 != 1"


; ==== 繰り返し ==============================
(dolist (v '(1 2 3)) (write v)(terpri))
; =>
; 1
; 2
; 3


; ==== 関数 ==============================

(defun add (x y)
    (+ x y))

(write (add 1000 1000))(terpri)
; => 2000


; ==== クラス ==============================

; 定義
(defclass color () (
    ; メンバと初期値とアクセサの設定
    (red    :initform 0 :accessor get-red)
    (green  :initform 0 :accessor get-green)
    (blue   :initform 0 :accessor get-blue)))

; メソッド
(defmethod print-color ((v color))
    (format t "color { red: ~x, green: ~x, blue: ~x }~%"
        (get-red v)
        (get-green v)
        (get-blue v)))

; インスタンス生成
(defparameter color-object (make-instance 'color))
(write color-object)(terpri)
; => #<COLOR {100195FB03}>

; メンバの値をアクセサから取得
(write (get-red color-object))(terpri)

; メソッドの実行
(print-color color-object)(terpri)
; => color { red: 0, green: 0, blue: 0 }
