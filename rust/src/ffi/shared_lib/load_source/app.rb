## gem "ffi"

# require 'ffi'

# module LibTest
#   extend FFI::Library
#   ffi_lib './target/debug/libfoo.dylib'
#   attach_function :foo, [], :int
# end

# LibTest::foo


require 'fiddle/import'

module FFI
  extend Fiddle::Importer

  #  Win  : libxxx.dll
  #  Mac  : libxxx.dylib
  # Linux : libxxx.so
  dlload File.expand_path('./../target/release/libfoo.so', __dir__)

  # C言語の形式で表記する
  extern 'unsigned int hello()'
end

FFI::hello()
