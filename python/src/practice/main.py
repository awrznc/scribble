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
