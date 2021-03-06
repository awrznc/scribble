CSS
===

CSS (Cascading Style Sheets) は特定の文書の体裁や見栄えを表現するために用いられるスタイルシート言語です。

## 記法

本チュートリアルではCSSの基本となる `セレクタ`、`プロパティ`、`値` の3つを扱います。

<object type="image/svg+xml" data="svg/css.svg"></object>

CSSはHTMLなどの文章と組み合わせて利用します。

例えば、以下のようなHTMLの要素が存在した場合、

```index.html
<p>Sample</p>
```

`<p>`の色`color`を青`blue`にする以下のようなCSSを上記のHTML文章に適用することで、`Sample` の文字列を青くすることができます。

```sample.css
p {
  color: blue;
}
```

適用がうまくいけば、以下のような青いSampleが表示されます。

<p style="color: blue;">Sample</p>


## 適用

HTMLにCSSを適用する手段はいくつか存在しますが、今回は`<link>`を利用する方法を紹介します。<br>
属性名`rel`の値に`stylesheet`を指定し、`href`の値にCSSファイルへのURI、またはパスを指定することで適用することができます。

```index.html
<html>
  <head>
    <link rel="stylesheet" href="sample.css">
    <title>Sample title</title>
  </head>
  <body>
    <p>Sample</p>
  </body>
</html>
```

また、`<link>`は`<head>`の中に記述するのが効率的であるためおすすめです。<br>
殆どの一般的なWebブラウザは、HTML文書を先頭から解析していきます。解析がファイルの読み込み、及びダウンロードの記述に到達すると、非同期でファイルの読み込み、及びダウンロードの処理が走ります。<br>
全体の処理時間は非同期の割合が大きいほど短くなるため、文章の先頭にCSSの読み込み処理を記述すると効率がよくなるわけです。


## セレクタ

セレクタに指定できるものはいくつかありますが、今回は以下の3つを扱います。

* `要素`
* `class`
* `id`

### 要素

要素のタグ名をセレクタに指定すると、CSSを適用させることができます。

```html
<p>Sample</p>
```

```css
p {
  color: red;
}
```


### class

CSSを適用させたい要素の属性に`class`を追加します。値は適当なものを入れておきましょう。

```html
<p class="sample">Sample</p>
```

セレクタにclassを指定する場合は、頭にドット`.`を付加します。

```css
.sample {
  color: red;
}
```

### id

CSSを適用させたい要素の属性に`id`を追加します。値は適当なものを入れておきましょう。

`id`は`class`と異なり、文書全体で一意な識別子となります。<br>
従って、ひとつの文章内で同じ値は利用できません。

```html
<p id="sample">Sample</p>
```

セレクタにidを指定する場合は、頭にシャープ`#`を付加します。

```css
#sample {
  color: red;
}
```

## 領域

要素の領域は以下の4つで構成されます。

* `content`
* `padding`
* `border`
* `margin`

contentを囲む余白がpaddingで、それを囲む枠線がborder、更にそれらを囲む余白がmarginです。<br>
文字や画像の距離が近い、もしくは遠いと感じた場合は、CSSでこのプロパティを調整してみましょう。

<object type="image/svg+xml" data="svg/css_space.svg"></object>


!> **実践**　青い矩形の作成（目安：15分）<br><br>
`<div>`を作成し、CSSを利用して青い矩形を表示してみましょう。<br>
余裕がある場合は、画面の中央に表示されるように調整してみましょう。<br><br>
ヒント：プロパティに`width`, `height`, `background`を利用してみましょう。作成例は<a href="sample\css_blue_square\index.html" target="_blank">こちら</a>。

!> **実践**　横並びのリスト（目安：15分）<br><br>
`<ul>`と`<li>`を利用してリストを作成したあと、CSSを利用して横並びにしてみましょう。<br>
ただし、リストを横並びにした際に、文字列同士がくっついてしまうのは読み辛いためNGです。余白を空けるようにしましょう。<br>
また、リストの先頭につく中点`・`も消すように調整してみましょう。<br>
余裕がある場合は、リストの文字列それぞれにリンクを持たせ、クリックすると適当なページに遷移するようにしてみましょう。<br><br>
ヒント：作成例は<a href="sample\css_side_by_side_list\index.html" target="_blank">こちら</a>。

!> **実践**　レストランのメニュー作成（目安：30分）<br><br>
前項にて、HTMLで作成したレストランのメニューにCSSを適用させて、背景などを調整してみましょう。<br>
余裕がある場合は、メニューの写真も追加してみましょう。

!> **実践**　Webページの模写（目安：60分）<br><br>
そろそろHTMLやCSSに慣れて来た頃だと思うので、好みのWebページを検索して、ソースコードを見ずに模写してみましょう。<br>
ブログやSNSは複雑なので、適当なコーポレートページを模写するのがおすすめです。
