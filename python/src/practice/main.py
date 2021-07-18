# === Class Example === === ===

class Sample:

    # メソッドは第一引数にそのクラスのインスタンスを表すオブジェクトを受け取る必要がある
    def print_sample(self):
        print('sample')

Sample().print_sample()
# => sample


# === Initialize Example === === ===

class InitializeExsampleClass:

    # インスタンス変数
    value = ''

    def __init__(self):
        print('initialize!')
        self.value = 'sample'

    def print_value(self):
        print(self.value)

isc = InitializeExsampleClass()
# => initialize!

isc.print_value()
# => sample


# === with Example === === ===
class WithExsampleClass():
    def __enter__(self):
        print("before")

    def __exit__(self, exc_type, exc_value, traceback):
        print("after")


# 処理の開始時と終了時に事前に定義しておいた処理を実行する
# Class側に事前に処理を実装しておく必要あり
with WithExsampleClass() as value:
    print('main')

# before -> main -> after の順番で表示される
