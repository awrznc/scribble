package Sample;

# インスタンス変数
$sample_value = 'value';

# コンストラクタ
sub new() {
    print "constructor.\n";
    my $self = {};
    my $package_name = "Sample";
    bless($self, $package_name);
    return $self;
}

# デストラクタ
sub DESTROY() {
    print "destructor.\n";
}

# メソッド
sub print_sample_value() {
    print $sample_value;
    print "\n";
    return $sample_value;
}
