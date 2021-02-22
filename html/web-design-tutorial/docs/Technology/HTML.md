HTML
===

HTML (HyperText Markup Language) は主にWebページを作成するための形式言語です。<br>
HTMLにはいくつかのバージョンが存在しますが、本チュートリアルではHTML5を学んでいきましょう。


## 記法

HTML形式の記法は `要素` の入れ子構造で構成されます。<br>
例外はありますが、基本的には以下のような記法になります。

<br><object type="image/svg+xml" data="svg/element.svg"></object><br><br>

注目してもらいたいのは、 `タグ` なる等比記号 `<` `>` で囲まれた記法が存在する点です。<br>
このタグで囲ったテキスト及び要素で入れ子の構造を表します。<br>
終了タグにはスラッシュ `/` が必要であることに注意してください。

```html
<!--
  div要素の中の
  div要素の中の
  div要素の中に
  "sample text"なるテキストが存在しているHTML
-->
<div>
  <div>
    <div>
      sample text
    </div>
  </div>
</div>
```

前項で学んだMarkdownでは記号に役割を持たせていましたが、HTMLではタグに一貫されています。

例えば、以下のようなMarkdownの例があるとします。

```markdown
## Sample

* sample
* sample
* sample
```

一方、HTMLでは以下のような表現になります。

```html
<h2>Sample</h2>
<ul>
  <li>sample</li>
  <li>sample</li>
  <li>sample</li>
</ul>
```

`##` が `<h2></h2>` に、 `*` が `<ul></ul>` と `<li></li>` で表現されるようになりました。

もし余裕がある場合は、以下の表を参考に、既存のMarkdownのファイルをHTMLに置き換えてみましょう。<br>
ちなみに、全ての記法をカバーしているわけではないので、目当ての記法が見当たらない場合は検索して自分で置き換えてみましょう。

名称 | 表示 | Markdown | HTML
--- | --- | --- | ---
段落         | <p style="margin: 0;">sample</p>    | `sample`     | `<p>sample</p>`
見出し1      | <h1 style="margin: 0;">sample</h1>  | `# sample`   | `<h1>sample</h1>`
見出し2      | <h2 style="margin: 0;">sample</h2>  | `## sample`  | `<h2>sample</h2>`
見出し3      | <h3 style="margin: 0;">sample</h3>  | `### sample` | `<h3>sample</h3>`
箇条書き     | <li>sample</li>                     | `* sample`   | `<ul><li>sample</li></ul>`
チェックボックス | <input type="checkbox">&nbsp;sample | `* [ ] sample` | `<input type="checkbox">sample`
ななめ       | *sample*                            | `*sample*`   | `<i>sample</i>`
強調         | **sample**                          | `**sample**` | `<b>sample</b>`
取り消し     | ~~sample~~                          | `~~sample~~` | `<s>sample</s>`
コード       | `sample`                            | <code>\`sample\`</code> | `<code>sample</code>`
引用         | <blockquote style="margin: 0;">sample</blockquote> | \> sample | `<blockquote>sample</blockquote>`
リンク       | [sample](/)                         | `[sample](/)`| `<a href="/">sample</a>`

## 枠組み

HTMLの基本的な記法を学んだので、早速Webページを作成してみましょう。<br>
まずはページ全体の枠組みとなる全体構造の最低限の要素を記述します。

```index.html
<html>
  <head>
    <title>Sample title</title>
  </head>
  <body>Sample body</body>
</html>
```

作成したHTMLファイルをダブルクリック、もしくはブラウザにドラッグ&ドロップすることで、以下のような内容が描画されるはずです。

<object type="image/svg+xml" data="svg/html.svg"></object>

うまく描画されたら、 `<body>` にいくつか任意の要素を追加してみましょう。

## 要素

要素 (element) とは、Webページを構成する部品です。

<object type="image/svg+xml" data="svg\html_element.svg"></object>

要素の細かい種類に関してはここで扱いきれないため、以下のMozillaさんのページを参照してください。<br>
https://developer.mozilla.org/ja/docs/Web/HTML/Element

?> **特殊文字**<br><br>
HTMLでは`<`、`>`、`"`、`'`、`&`の5つの文字を特殊文字と呼びます。<br>
これらの文字はHTMLの記法で利用する文字であるため、そのままではテキストとして利用することができません。<br>
そのような場合は「文字参照」を利用することで特殊文字の代替を行うことができます。半角スペースも同じ方法での代替が可能です。<br>
https://developer.mozilla.org/ja/docs/Learn/HTML/Introduction_to_HTML/Getting_started#entity_references_including_special_characters_in_html

## 属性

属性 (attribute) とは、さまざまな方法で要素を構成したり、動作を調整したりする追加の値です。

属性の細かい種類に関してはここで扱いきれないため、以下のMozillaさんのページを参照してください。<br>
https://developer.mozilla.org/ja/docs/Web/HTML/Attributes


?> **デベロッパーツール**<br><br>
ブラウザに表示されているWebページのHTML文章は、デベロッパーツールから調査・編集することができます。<br>
さっそくキーボードの`F12`ボタンを押して、`Sources`タブを押してみましょう。そのページで利用しているソースコードを見ることができるはずです。

!> **実践**　表の作成（目安：10分）<br><br>
HTMLを利用して表を作成してみましょう。<br>
表の構成要素は自由ですが、特に思いつかない場合は、ここ数日の天気を表にしてみましょう。列は日付に、行は地域名称にしてみましょう。<br><br>
ヒント: `<table>` を利用してみましょう。

!> **実践**　レストランのメニュー作成（目安：20分）<br><br>
前項にて、Markdownで作成したレストランのメニューをHTMLに書き換えてみましょう。<br>
余裕がある場合は、メニューの写真も追加してみましょう。
