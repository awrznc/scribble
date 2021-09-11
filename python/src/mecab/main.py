import MeCab

class getWords():

    def readText(self, filename):
        # ファイルを展開
        with open(filename + '.txt', 'r', encoding='utf-8') as f:
            tText = f.read()
            f.close()
        return tText


    def mecab(self, text):
        mThings = MeCab.Tagger('Ochasen')
        node = mThings.parseToNode(text)

        # 形態素解析
        keywords = []
        while node:
            if node.feature.split(',')[0] == '名詞':
                keywords.append(node.surface)
                # print(node.surface)
                # print(node.feature)
            node = node.next
        return keywords


def getCommonNoun():

    gWords1 = getWords()
    gWords2 = getWords()

    text1 = gWords1.readText('text1')
    text2 = gWords2.readText('text2')

    mecabWords1 = gWords1.mecab(text1)
    mecabWords2 = gWords2.mecab(text2)

    # print(mecabWords1)
    # print(mecabWords2)

    setMw1 = set(mecabWords1)
    setMw2 = set(mecabWords2)

    andWords = setMw1 & setMw2

    print(andWords)


def parseText():
    text = "吾輩は猫である。"
    mThings = MeCab.Tagger('Ochasen')
    node = mThings.parseToNode(text)
    while node:
        print(node.surface)
        node = node.next


# 共通の名詞を取得する
getCommonNoun()
# => {'事', '猫'}

# テキストを要素に分解する
parseText()
# =>
# 吾輩
# は
# 猫
# で
# ある
# 。
