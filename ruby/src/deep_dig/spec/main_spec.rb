require_relative "./../main.rb"

describe '#deep_dig' do
  describe 'blackbox' do # 第一引数と第二引数のみ組み合わせテスト
    test_cases = [ nil, true, false, 9999, 'hoge', [], {} ]
    test_cases.each do |test_case_1|
      context "第一引数に #{test_case_1.class} 型の値( #{test_case_1} )を渡した場合" do
        test_cases.each do |test_case_2|
          context "第二引数に #{test_case_2.class} 型の値( #{test_case_2} )を渡した場合" do
            it 'エラーが発生しない' do
              deep_dig(test_case_1, test_case_2)
            end
          end
        end
      end
    end
  end

  describe 'whitebox' do
    let(:hash) do
      { hoge: { hoge: "hoge", huga: "hoge" } }
    end
    context '第二引数と同じ文字列のキーが存在するハッシュを第一引数に渡した場合' do
      it '返り値の型が配列である' do
        expect( deep_dig(hash, 'hoge').class ).to eq( Array )
      end
      it 'キーの値が配列で取得できる' do
        expect( deep_dig(hash, 'hoge') ).to eq([{:hoge=>"hoge", :huga=>"hoge"}, "hoge"])
      end
    end
  end
end
