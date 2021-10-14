# bashの配列は0から始まるが
# zshの配列は1から始まる
array=('ABC' 'DEF' 'GHI')
echo $array[2]
# => DEF
