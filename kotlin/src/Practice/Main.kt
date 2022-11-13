fun increment( value: Int ): Int {
    return value + 1
}

class Color {
    var red: Int = 0
    var green: Int = 0
    var blue: Int = 0
    init {
        red = 255
    }
    fun gray(): Int {
        return ( red + green + blue ) / 3
    }
}

fun main( args: Array<String> ) {

    // 出力
    println("Hello, Kotlin!")
    // => Hello, Kotlin!

    // 変数
    val value: Int = 2
    println( "value: $value" )
    // => value: 2

    // 四則演算
    val calc_result = 2 + 3 * 3 / 2.0 - 0
    println( "calc_result: $calc_result" )
    //  => calc_result: 6.5

    // 配列
    val array = arrayOf( 3, 2, 1 )
    println( array.contentToString() )
    // => [3, 2, 1]
    println( args.contentToString() )
    // => [] ( 引数によって変化する )

    // リスト
    val list = mutableListOf( 6, 5, 4 )
    // 中身を変更しない場合はlistOfを利用する
    // val list = listOf( 6, 5, 4 )
    list.add( 8 )
    println( list )
    // => [6, 5, 4, 8]

    // 条件分岐
    if ( value == 2 ) {
        println( "passed!" )
    }

    // 繰り返し
    println("list:")
    for (v in list) {
        println(" v: $v")
    }
    // list:
    //  v: 6
    //  v: 5
    //  v: 4
    //  v: 8

    // 関数
    val returned_value = increment( value )
    println( "returned_value: $returned_value" )
    // => returned_value: 3

    // クラス
    val color = Color()
    println( "gray: ${color.gray()}" )
    // => gray: 85
}
