# ========== XMLクラスを利用する場合 ========== ========== ==========

# PowerShell は XMLクラスが存在する
# XMLクラスを利用してXMLの形式で書かれた文字列をパースすることができる

# index.xml からXML型でオブジェクトを取得
$XML = [xml](Get-Content .\index.xml)

# Sample.HogeElem を表示
${XML}.Sample.HogeElem
# =>
# attr #text
# ---- -----
# Red  HogeText

# 属性名を指定して属性値を表示
${XML}.Sample.HogeElem.GetAttribute("attr")
# => Red

# テキストを表示
${XML}.Sample.HogeElem.'#text'
# => HogeText


# ========== XPathNavigator を利用する場合 ========== ========== ==========

# XMLクラスの持つ CreateNavigator() メソッドを利用すると
# XMLオブジェクトを XPathNavigator オブジェクトに変換することができる
# つまりXPathの記述を利用して値を取得することができる

# XPathNavigator オブジェクトを取得
$XPN = ${XML}.CreateNavigator()

# Sample以下の要素のテキストを全て表示
${XPN}.Select("/Sample/*").Value
# =>
# HogeText
# FugaText

# Sample以下の要素において、属性 attr の値が Blue であるテキストを全て表示
${XPN}.Select("/Sample/*[@attr = 'Blue']").Value
# => FugaText
