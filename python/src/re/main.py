import re

target = 'hello'

reg = re.compile(r'hel(%s)o' % '.')
result = reg.search(target)

print(result)
# => <re.Match object; span=(0, 5), match='hello'>

print(result.groups()[0])
# => l
