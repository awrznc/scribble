use proc_macro::TokenStream;

#[proc_macro]
pub fn return_input(input: TokenStream) -> TokenStream {
    input.to_string().parse().unwrap()
}

#[proc_macro_attribute]
pub fn print_args(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("args: {}\ninput: {}", args, input);
    input
}

#[derive(Debug)]
enum Error {
    NotStruct,
    FailedGetStructName,
    FailedGetStructContents,
    FailedGetFieldType,
}

struct KeyValue {
    key: String,
    value: String,
}

struct ParseResult {
    name: String,
    contents: Vec<KeyValue>,
}

fn parse_struct(input: TokenStream) -> Result<ParseResult, Error> {
    let mut iter = input.into_iter();

    // 構造体であるかどうかの確認
    if iter.next().ok_or(Error::NotStruct)?.to_string() != "struct" {
        return Err(Error::NotStruct);
    }

    // 構造体の中身の取得
    let mut struct_contents: ParseResult = ParseResult {
        name: iter.next().ok_or(Error::FailedGetStructName)?.to_string(),
        contents: vec![],
    };
    let contents = iter.next().ok_or(Error::FailedGetStructContents)?;
    if let proc_macro::TokenTree::Group(value) = contents {
        let mut ii = value.stream().into_iter();

        // フィールドの数だけ繰り返し処理
        while let Some(field_name) = ii.next() {
            ii.next();
            let field_type = ii.next().ok_or(Error::FailedGetFieldType)?;
            ii.next();
            struct_contents.contents.push(KeyValue {
                key: field_name.to_string(),
                value: field_type.to_string(),
            });
        }
    }

    Ok(struct_contents)
}

#[proc_macro_derive(Work)]
pub fn print_work(input: TokenStream) -> TokenStream {
    let parsed_struct = parse_struct(input).unwrap();
    let contents_printer = parsed_struct
        .contents
        .iter()
        .map(|content| {
            format!(
                "println!(\"name: {}, type: {}\");\n",
                content.key, content.value
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let fn_print_work = stringify!(
        pub fn print_work(&self) {
            println!("work!")
        }
    );

    let fn_print_contents = format!(
        "{} {{ {} }}",
        stringify!( pub fn print_contents(&self) ),
        contents_printer
    );

    let result_string = format!(
        "impl {} {{ {} {} }}",
        parsed_struct.name, fn_print_work, fn_print_contents,
    );

    dbg!(&result_string);

    result_string.parse().unwrap()
}
