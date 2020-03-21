use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Internationalization {
    ja: String,
    en: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Talk {
    scene_hello: Vec<Internationalization>
}

fn main() {
    load_file();
}

// ファイルの読み込み
fn load_file() -> u8 {

    // コンパイル時にファイルを読み込んで文字列に置き換える
    let file_contents = include_str!("./../assets/talk.yml");

    // yamlを構造体にパース
    let data: Talk = serde_yaml::from_str(&file_contents).expect("load file error");

    // 表示
    println!("{:#?}", data);
    // =>
    // Talk {
    //     scene_hello: [
    //         Internationalization {
    //             ja: "こんにちは!",
    //             en: "hello!",
    //         },
    //     ],
    // }

    return 0;
}
