#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Character<'a> {
    pub name: &'a str,
    pub cost: usize,
}

pub type Characters = std::collections::HashMap<
    &'static str,
    std::vec::Vec<Character<'static>>
>;

macro_rules! json_parse {
    ($path:tt) => {
        {
            let json_string: Vec<&str> = include_str!($path).split('"').collect();

            let mut strs: Vec<&str> = Vec::new();
            for (i, val) in json_string.iter().enumerate() {
                if (i + 1) % 2 == 0 {
                    strs.push(val);
                }
            }

            let mut characters: Characters = std::collections::HashMap::new();
            let mut keykey: &str = "";
            let mut node: &str = "";
            let mut cost: usize = 0;
            let node_str: &str = "node";
            let cost_str: &str = "cost";
            let mut is_key: bool = true;
            let mut is_align: (bool, bool) = (false, false);
            let mut is_node: bool = false;
            let mut is_cost: bool = false;
            let mut char_vec: Vec<Character> = Vec::new();

            for iiii in strs.iter() {
                if is_key == true {
                    is_key = false;
                    keykey = iiii;
                } else if iiii == &node_str {
                    is_node = true;
                } else if iiii == &cost_str {
                    is_cost = true;
                } else if is_node == true {
                    node = iiii;
                    is_align.0 = true;
                    is_node = false;
                } else if is_cost == true {
                    cost = iiii.parse::<usize>().unwrap();
                    is_align.1 = true;
                    is_cost = false;
                } else {
                    characters.insert(keykey, char_vec.clone());
                    char_vec = Vec::new();
                    keykey = iiii;
                    continue;
                }
                if is_align == (true, true) {
                    char_vec.push(Character { name: node, cost: cost });
                    is_align = (false, false);
                }
            }

            characters.insert(keykey, char_vec.clone());

            characters
        }
    };
}

fn main() {
    // get();
    let c: Characters = json_parse!("data.json");
    println!("{:?}", c);
}

// fn get<'k>() -> Characters<'k> {
//     let json_string = include_str!("data.json").to_string();
//     let remove_nl_string: String = json_string.chars().filter(|c| !c.is_whitespace()).collect();

//     println!("{}", remove_nl_string);

//     // let mut vec: Vec<String> = Vec::new();
//     // let mut is_string = false;
//     // let mut is_primary_key = true;
//     // let mut primary_key = "";

//     // let mut is_node = false;
//     // let mut is_cost = false;
//     // let mut node: &str = "";
//     // let mut cost: usize = 0;
//     // // let mut charchar = Character { node: "a", cost: 0 };
//     // let result = remove_nl_string.chars().fold(
//     //     ("", vec),
//     //     |result, character|
//     //     {
//     //         match character {
//     //             '[' => {
//     //                 is_primary_key = false;
//     //             },
//     //             ']' => {
//     //                 is_primary_key = true;
//     //             },
//     //             '"' => {

//     //             if is_string == true {
                    
//     //                 // key.push(vec.clone().concat());

//     //                 // 初期化
//     //                 // result.1 = Vec::new();
//     //                 is_string = false;
//     //             } else {
//     //                 is_string = true;
//     //             }
//     //         },
//     //             _ => {
//     //                 if is_string == true {
//     //                     // result.1.push(character.to_string());
//     //                 }
//     //             }
//     //         }
//     //         result
//     //     }
//     // );

//     // println!("{:?}", result);


//     let mut characters: Characters = std::collections::HashMap::new();
//     let mut key: Vec<String> = Vec::new();
//     let mut costs: Vec<usize> = Vec::new();
//     {
//         let mut vec: Vec<String> = Vec::new();
//         let mut is_string = false;
//         let mut is_primary_key = true;
//         let mut primary_key = "";
//         let mut is_array = false;
//         // let mut charchar = Character { node: "a", cost: 0 };

//         let mut is_node = false;
//         let mut is_cost = false;
//         let mut node: &str = "";
//         let mut cost: usize = 0;
//         // let mut word: &str = "";

        

//         for character in remove_nl_string.chars() {

//             // let wordword = vec.clone().concat();

//             match character {
//                 '}' => {
//                     if is_primary_key == false {
//                         // println!("{:?}", node.clone());
//                         println!("{:?}", Character { name: node, cost: cost });
//                     }
//                 }
//                 '[' => {
//                     is_primary_key = false;
//                 },
//                 ']' => {
//                     is_primary_key = true;
//                 },
//                 '"' => {

//                     if is_string == true {
                        
//                         key.push(vec.clone().concat());
//                         if is_primary_key == true {
//                             let tmp = vec.clone().concat();
//                             primary_key = &tmp;
                            
//                             // characters.insert(
//                             //     vec.clone().concat(),
//                             //     [Character {name: "aaa", cost: 0}].to_vec()
//                             // );
//                             println!("primary_key: {:?}", primary_key);
//                         } else {

//                             if vec.concat() == "node" {
//                                 is_node = true;  is_cost = false
//                             } else if vec.concat() == "cost" {
//                                 is_node = false;  is_cost = true
//                             } else {
//                                 if is_node == true {
//                                     // node = &vec.clone().concat();
//                                 } else if is_cost == true {
//                                     // costs.push(vec.clone().concat().parse::<usize>().unwrap());
//                                     // cost = word.parse::<usize>().unwrap();
//                                 }
//                             }
//                             // let wwww = &mut word;
//                             // wwww = &mut &(vec.concat())[..];
//                             // word = &(vec.concat())[..];
//                             // *word = &mut &(vec.concat())[..];
//                             // match wwww {
//                             //     "node" => { is_node = true;  is_cost = false },
//                             //     "cost" => { is_node = false; is_cost = true },
//                             //     _ => { 
//                             //         if is_node == true {
//                             //             node = word.clone();
//                             //         } else if is_cost == true {
//                             //             cost = word.parse::<usize>().unwrap();
//                             //         }
//                             //     }
//                             // }
//                             // println!("{:?}, {:?}", node, cost);
//                             // println!("node: {}, code: {}", is_node, is_cost);
//                         }

//                         // 初期化
//                         vec = Vec::new();
//                         is_string = false;
//                     } else {
//                         is_string = true;
//                     }
//                 },
//                 _ => {
//                     if is_string == true {
//                         vec.push(character.to_string());
//                     }
//                 }
//             }
//         }
//     }

//     println!("{:?}", key);
//     println!("{:?}", characters);
//     println!("{:?}", costs);

//     let mut characters: Characters = std::collections::HashMap::new();
//     let mut keykey: &str = "";
//     let mut node: &str = "";
//     let mut cost: usize = 0;
//     let mut node_str: &str = "node";
//     let mut cost_str: &str = "cost";
//     let mut is_key: bool = true;
//     let mut is_align: (bool, bool) = (false, false);
//     let mut is_node: bool = false;
//     let mut is_cost: bool = false;
//     let mut is_a: bool = false;
//     let mut char_vec: Vec<Character> = Vec::new();
//     let strs: Vec<&str> = key.iter().map(AsRef::as_ref).collect();
//     // println!("{:?}", strs);

//     for iiii in strs.iter() {
//     // strs.iter().for_each(|iiii|{
//         if is_key == true {
//             is_key = false;
//             keykey = iiii;
//         } else if iiii == &node_str {
//             is_node = true;
//         } else if iiii == &cost_str {
//             is_cost = true;
//         } else if is_node == true {
//             node = iiii;
//             is_align.0 = true;
//             is_node = false;
//         } else if is_cost == true {
//             cost = iiii.parse::<usize>().unwrap();
//             is_align.1 = true;
//             is_cost = false;
//         } else {
//             characters.insert(keykey, char_vec.clone());
//             char_vec = Vec::new();
//             keykey = iiii;
//             continue;
//         }
//         println!("====");
//         if is_align == (true, true) {
//             println!("{:?}", Character { name: node, cost: cost });
//             char_vec.push(Character { name: node, cost: cost });
//             is_align = (false, false);
//         }
//     // });
//     }

//     println!("{:?}", characters);
//     // println!("{:?}", char_vec);

//     return characters;
// }
