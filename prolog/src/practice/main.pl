
:- encoding(utf8).
% UTF8 を明示的に指定しないと実行時に Warning が表示されてしまう

% この命令を含むファイルのロードが完了した後に対象を呼び出す
% 今回は main を呼び出す
:- initialization(main).
% 以下の記述方法でもOK
% :- initialization main.


% === 繰り返しに利用する関数 ===

% 以下のような順番で実行される:
%   loop_example -> loop(X) -> loop(3)

% C言語だと以下のような処理になる:
%   for(int x=1; x==3; x++) printf("%d", x);

loop(3) :- write(3),nl.

loop(X) :-
    write(X),nl,
    Y is X + 1,
    loop(Y).
loop(_).

loop_example :-
    % 1-3を表示するサンプル
    loop(1).
loop_example.

% ==================================


% main
main :-

    % 標準出力
    format('format example.~n'),
    % => format example.
    write('write example.~n'),
    % => write example.~n
    % write の場合は ~n がそのまま出力される
    % nlで改行できる
    nl,

    % 四則演算
    is(A, 2+3*3/2-0),
    format(A),
    format('~n'),
    % => 6.5
    % 乗除が優先される

    % 比較
    % =:= ... 一致
    % =\= ... 不等
    % <   ... 超過
    % >   ... 未満
    % =<  ... 以上
    % >=  ... 以下
    (2 * 2) =:= (2 + 2),

    % 条件分岐 (if else)
    % (比較) -> (if) ; (else)
    is(B, 9),
    format('B: 9~n'),
    B > 10 -> format('B is bigger than 10.~n') ; format('B is smaller than 10.~n'),
    % => B is smaller than 10.
    % B には 9 が代入されているため、10より小さい判定になる。

    % 繰り返し (loop)
    % 再帰を利用する
    loop_example,

    % 終了
    halt(0).
main.
