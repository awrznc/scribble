import zlib

# 圧縮の対象
data = b'sample string'

# 圧縮
compressed = zlib.compress(data)

# 展開
print(zlib.decompress(compressed))
