describe '' do
  shared_examples 'equal' do |a, b, c|
    context "case: #{a}" do
      it "equal" do
        expect(b).to eq(c)
      end
    end
  end

  it_behaves_like 'sample_1' 1 1
  it_behaves_like 'sample_2' 'example' 'example'
end
