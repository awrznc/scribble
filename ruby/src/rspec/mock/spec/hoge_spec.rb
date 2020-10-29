require_relative "spec_helper.rb"
require_relative "./../hoge.rb"

describe 'Hoge' do

  describe '#initialize' do
    context "初期化の引数に文字列を渡した場合" do

      it 'エラーが発生しない' do
        Hoge.new('print fuga.')
      end

    end
  end

  describe '#print' do
    context "引数に何も指定せずに実行した場合" do

      let (:string)       { 'print fuga.'     }
      let (:mock_string)  { 'print piyo.'      }
      let (:hoge)         { Hoge.new(string)  }

      it '初期化で渡した引数+改行が標準出力される' do
        expect { hoge.print() }.to output("#{string}\n").to_stdout
      end

      it '#printをmock化する例' do

        # printメソッドが呼ばれた際の挙動を定義
        expect(hoge).to receive(:print) do
          puts mock_string
        end.and_return(mock_string)

        # Test
        expect { hoge.print() }.to output("#{string}\n").to_stdout
        # =>
        # Failure/Error: expect { hoge.print() }.to output("#{string}\n").to_stdout
        #   expected block to output "print fuga.\n" to stdout, but output "print piyo\n"
        #   Diff:
        #   @@ -1 +1 @@
        #   -print fuga.
        #   +print piyo
      end

      it 'Hogeオブジェクト自体をダミーオブジェクト化する例' do

        # create Hoge client mock object（一引数目はなんでもいい）
        hoge_object = double('hoge dummy object', :print => mock_string)

        # Test
        expect( hoge_object.print() ).to eq("#{string}")
        # => 
        # Failure/Error: expect( hoge_object.print() ).to eq("#{string}")
        #   expected: "print fuga."
        #        got: "print piyo."
        #   (compared using ==)
      end

      it 'ダミーオブジェクトにメソッドを追加する例' do
        hoge_object = double('foo', :print => mock_string)

        # 非推奨
        # hoge_object.stub(:merge_print) do
        #   puts mock_string
        #   mock_string
        # end

        expect(hoge_object).to receive(:merge_print) do
          puts mock_string
        end.and_return(mock_string)

        # Test
        expect( hoge_object.merge_print() ).to eq("#{string}")
        # =>
        # Failure/Error: expect( hoge_object.merge_print() ).to eq("#{string}")
        #   expected: "print fuga."
        #        got: "print piyo."
        #   (compared using ==)
      end

      it 'ダミーオブジェクトを利用して実装を書き換える例' do
        foo_mock = double('Foo client')
        expect(foo_mock).to receive(:get_bar) 

        # 実装を書き換える
        expect(hoge).to receive(:foo) do
          puts mock_string
        end.and_return(foo_mock)

        # Test
        expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        # =>
        # Failure/Error: expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        #   expected block to output "print fuga.\n" to stdout, but output "print piyo.\nprint fuga.\n"
        #   Diff:
        #   @@ -1 +1,2 @@
        #   +print piyo.
        #   print fuga.
      end

      it '既存オブジェクトの挙動を書き換える例(インスタンスメソッド編)' do

        expect_any_instance_of(Foo).to receive(:get_bar).and_return('dummy')

        # Test
        expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        # =>
        # Failure/Error: expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        #   expected block to output "print fuga.\n" to stdout, but output "print fuga.dummy\n"
        #   Diff:
        #   @@ -1 +1 @@
        #   -print fuga.
        #   +print fuga.dummy
      end

      it '既存オブジェクトの挙動を書き換える例(クラスメソッド編)' do

        expect(Foo).to receive(:new).and_return('dummy')

        # Test
        expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        # =>
        # Failure/Error: expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        #   NoMethodError:
        #   undefined method `get_bar' for "dummy":String

        # Foo.newから"dummy"なる文字列が返るようになったため
        # Stringクラスにget_barメソッドがないよと怒られている
      end

      it '継承とオーバーライドを利用する例' do

        expect(Foo).to receive(:new).and_return(NeoFoo.new(string))

        # Test
        expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        # => 
        # Failure/Error: expect { hoge.merge_print() }.to output("#{string}\n").to_stdout
        #   expected block to output "print fuga.\n" to stdout, but output "print fuga.dummy\n"
        #   Diff:
        #   @@ -1 +1 @@
        #   -print fuga.
        #   +print fuga.dummy
      end

    end
  end

end
