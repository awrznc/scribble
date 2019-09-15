
// ================================================================================================
fn main() {
    // 標準出力
    println!("Hello, world!");

    // 変数束縛
    variable();

    print_number(5);

    println!("{}", add_one(5));
    // diverges();

    // 関数ポインタ
    let f: fn(i32) -> i32 = add_one;
    println!("{}", f(5));

    // primitive
    primitive();

    // if
    if_fn();

    // loop
    loop_fn();

    // struct
    struct_fn();

    // enum
    enums_fn();

    // match
    match_fn();

    // pattern
    pattern_fn();

    // method-syntax
    method_syntax();

    // vector
    vector_fn();

    // string
    string();

    // generic
    generic();

    // trait
    trait_fn();
}

// ================================================================================================
fn variable() {
    // 変数束縛(型推論)
    let _a = 5;

    // 変数束縛(静的な型付)
    let _b: i32 = 5;

    // 変数束縛(ミュータル(変更可))
    let mut _c: i32 = 5;
    _c = 10;

    // 変数の表示
    let d: i32 = 5;
    println!("d: {}", d);

    // スコープ
    let e: i32 = 5;
    {
        println!("e: {}", e); // => 5
        // シャドーイング
        let e = "hoge";
        println!("e: {}", e); // => 12
    }
    println!("e: {}", e); // => 5
}

// ================================================================================================
fn print_number(x: i32) {
    println!("num is: {}", x);
}

// ================================================================================================
fn add_one(x: i32) -> i32 {
    return x + 1;
}

// ================================================================================================
// RUST_BACKTRACE=1 cargo run
// fn diverges() -> ! {
//     panic!("This function never returns!");
// }

// ================================================================================================
fn primitive() -> bool{

    // boolean
    let _x = true;
    let _y: bool = false;

    // char
    let _x = 'x';
    let _two_hearts: char = '💞';

    // integer
    let _x: usize = 1234567;

    // array
    let _a = [1, 2, 3];
    let _a = [0; 20];
    // println!("{}", a); // error
    println!("{}, {}, {}", _a.len(), _a[0], &_a[1..3].len());

    // tuple
    let _tuple = (1, "hello");
    let _tuple: (i32, &str) = (1, "hello"); // 上に同じ
    println!("{}", _tuple.1);

    return true;
}

// ================================================================================================
fn if_fn() {
    let x = 5;

    if x == 5 {
        println!("x は 5 です!");
    } else if x == 6 {
        println!("x は 6 です!");
    } else {
        println!("x は 5 でも 6 でもありません :(");
    }

    let _y = if x == 5 { 10 } else { 15 }; // y: i32
    println!("{}", _y);
}

// ================================================================================================
fn loop_fn() {
    // loop {
    //     println!("Loop forever!");
    // }

    // while もある

    // for
    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    let lines = ["one", "two"];
    for (linenumber, line) in lines.iter().enumerate() {
        println!("{}: {}", linenumber, line);
    }

    'outer: for x in 0..4 {
        'inner: for y in 0..4 {
            if x % 2 == 0 { continue 'outer; } // x のループを継続
            if y % 2 == 0 { continue 'inner; } // y のループを継続
            println!("x: {}, y: {}", x, y);
        }
    }
}


// ================================================================================================
struct Point3d {
    _x: i32,
    _y: i32,
    _z: i32,
}
fn struct_fn() {

    // struct
    let _origin = Point3d { _x: 0, _y: 0, _z: 0 };
    let _point = Point3d { _y: 1, .. _origin };
    println!("The origin is at ( {}, {}, {} )", _point._x, _point._y, _point._z);

    // tuple構造体なるものがあるらしい。要素が1つだけの場合などに便利だとか
    // unit-like構造体なるものもあるらしい
}


// ================================================================================================
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn enums_fn() {
    let x: Message = Message::Move { x: 3, y: 4 };

    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
}

// ================================================================================================
fn match_fn() {
    let x: i32 = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }
}

// ================================================================================================
fn pattern_fn() {
    let _x = 2;
    match _x {
        1 => println!("one"),
        2 | 3 => println!("two over"),
        _ => println!("anything"),
    }

    let x = 'x';
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c),
        'c' => println!("x: {} c: {}", x, c),
        _ => println!("other"),
    }

    println!("x: {}", x);

    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 11, y: 12 };

    match origin {
        Point { x: x_copy, .. } => println!("{}", x_copy),
    }

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }

    let x = OptionalTuple::Value(1, 2, 3);
    // let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }

    let x = 5;

    match x {
        ref r => println!("Got a reference to {}", r),
    }

    let x = '💅';

    match x {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else"),
    }
}

// ================================================================================================
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}

fn method_syntax() {
    let c = Circle::new(0.0, 0.0, 2.0);
    // let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

     let d = Circle::new(0.0, 0.0, 2.0).grow(2.0).area();
     println!("{}", d);
}

// ================================================================================================
fn vector_fn() {
    let v = vec![1, 2, 3, 4, 5];

    println!("The third element of v is {}", v[2]);
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}

// ================================================================================================
fn string() {
    let mut s = "foo
        bar".to_string();
    s.push_str("ほげほげ\n");
    println!("{}", s);

    for c in s.chars() {
        print!("{}, ", c);
    }

    let dog = "hachiko";
    let hachi = &dog[0..5];
    println!("{}", hachi);
}

// ================================================================================================
enum Option<T> {
    Some(T),
    None,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn generic() {
    let x = Some(5);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// ================================================================================================
trait HasArea {
    fn area(&self) -> f64;
}

struct NeoCircle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for NeoCircle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn trait_fn() {
    let c = NeoCircle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c);
    print_area(s);
}
