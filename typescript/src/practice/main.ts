// value を文字列型として宣言・代入
let value: string = "String example."


// 出力
console.log(value);


// interface
interface ColorInterface {
    red: number;
    green: number;
    blue: number;

    printRGB: () => Array<Number>;
}


// class
class Color implements ColorInterface {
    red: number;
    green: number;
    blue: number;

    constructor(red: number, green: number, blue: number) {
        this.red = red;
        this.green = green;
        this.blue = blue;
    }

    printRGB() {
        console.log(`red: ${this.red}, green: ${this.green}, blue: ${this.blue}`);
        return [this.red, this.green, this.blue]
    }
}

let colorObject: Color = new Color(255, 0, 0);
colorObject.printRGB();


// Union
type UnionExample = number | string;

let unionExample: UnionExample = 0;

console.log(unionExample);


// Generics
type Result<T> = {
    value: T;
    message: string;
};

let genericsExample: Result<number> = {
    value: 2,
    message: "success",
};

console.log(genericsExample.value);


// リテラル型
type State = "success" | "error";
// let stateExample: State = "err"; // error TS2322: Type '"err"' is not assignable to type 'State'.
let stateExample: State = "error";

console.log(stateExample);
