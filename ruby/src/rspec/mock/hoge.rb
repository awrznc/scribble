class Foo
  def initialize(string)
    @bar = string
  end

  def get_bar()
    return @bar
  end
end

class Hoge
  def initialize(string)
    @fuga = string
  end

  def print
    puts @fuga
    return @fuga
  end

  def merge_print()
    bar = foo.get_bar()
    result = "#{@fuga}#{bar}"
    puts result
    return result
  end

  def foo
    Foo.new('baz')
  end
end
