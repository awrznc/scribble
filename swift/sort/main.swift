// 右の要素が小さい場合はtrueを返す
func isBig(valueA: String, valueB: String) -> Bool {
    let codesA: String.UTF8View = valueA.utf8
    let codesB: String.UTF8View = valueB.utf8
    
    for (index, charA) in codesA.enumerated() {
        if Int(charA) < Int(Array(codesB)[index]) {
            return true
        } else if Int(charA) > Int(Array(codesB)[index]) {
            return false
        }
    }
    return true
}

// 実際の処理
let list: [String] = ["aaa", "aAa", "aaA", "AAA", "bbb"]
let result = list.sorted(by: { a, b -> Bool in
    return isBig(valueA: a, valueB: b)
})
print( result )
// => ["AAA", "aAa", "aaA", "aaa", "bbb"]
