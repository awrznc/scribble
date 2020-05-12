class SampleClass
end

SampleClass.class_eval do
  method_name = 'sample_method'
  define_method method_name do
    return 'dynamic'
  end
end


obj = SampleClass.new
p obj.sample_method
p obj.send("sample_method")
