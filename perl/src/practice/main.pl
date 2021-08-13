# UTF8の利用
use utf8;


# 標準出力
print "print example.\n";


# 文字列の連結（ドット`.`で連結する）
print 'merge'." string.\n";
# => merge string.


# 四則演算
print 2 + 3 * 3 / 2 - 0;
print "\n";
# => 6.5


# 変数
my $value = 1 + 1;
print $value;
print "\n";


# 条件分岐
if (2 == 2) {
    print "2 == 2\n";
} else {
    print "2 != 2\n";
}


# 繰り返し

## while (条件が正である限りスコープをループする)
while (2 == 2) {
    print "while example.\n";

    # C言語だとbreakに相当
    last;
}

## until (条件が負である限りスコープをループする)
until (2 != 2) {
    print "until example.\n";

    # C言語だとbreakに相当
    last;
}

## for (スコープ内で利用できる変数とループの条件を宣言する)
for ($i = 0; $i < 1; $i++) {
    print "i = $i \n";
}


# サブルーチン
sub print_subroutine() {
    print "subroutine\n";
}

print_subroutine;


# 正規表現
if ("hogehoge" =~ /geho/ ) {
    print("include geho.\n");
}


# クラス
# ライブラリへのパスを通さないとエラーになる
# PERL5LIB="$PERL5LIB:$(pwd)/src/practice/" perl ./src/practice/main.pl
use Sample;

## new
$sample = Sample->new();

## method
$sample->print_sample_value();

## delete
undef $sample;
