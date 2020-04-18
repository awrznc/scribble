use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Page<'a> {
    name: &'a str,
    cost: usize,
}

impl Ord for Page<'_> {
    fn cmp(&self, other: &Page) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Page) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_low_cost<'page>(
    adjacent_list: &'page std::collections::HashMap<&str, std::vec::Vec<Page<'_>>>,
    start: &'page str,
    goal: &str
) -> Option<usize> {

    // set route info
    let mut hash: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for key in adjacent_list.keys() { hash.insert(key, usize::MAX); }

    // set start
    let mut heap = BinaryHeap::new();
    heap.push(Page { cost: 0, name: start });
    hash.insert(start, 0);

    // search shortest distance
    while let Some(Page { cost, name }) = heap.pop() {
        println!("total_cost: {}, name: {}", cost, name);

        // 最短経路を見つけることができたら終了
        if name == goal { return Some(cost); }

        // すでにより良い経路を見つけていたらスキップ
        if cost > hash[name] { continue; }

        for point in &adjacent_list[name] {
            let next_point = Page { cost: cost + point.cost, name: point.name };
            if next_point.cost < hash[next_point.name] {
                heap.push(next_point);
                hash.insert(next_point.name, next_point.cost);
            }
        }
    }
    None
}

macro_rules! json_parse {
    ({ $($key:tt : $tt:tt),+ }) => {
        {
            let mut dictionary = ::std::collections::HashMap::new();
            $(
                dictionary.insert($key, array_parse!($tt));
            )+
            dictionary
        }
    };
}

macro_rules! array_parse {
    ([ $($tt:tt),* ]) => {
        {
            // use if array is not empty
            #[allow(unused_mut)]
            let mut page = Vec::new();
            $(
                page.push( hash_parse!($tt) );
            )*
            page
        }
    };
}

macro_rules! hash_parse {
    ({ $($key:tt : $value:expr),+ }) => {
        {
            let mut node: &str = "";
            let mut cost: usize = 0;
            $(
                match $key {
                    "node" => node = $value,
                    "cost" => cost = $value.parse::<usize>().unwrap(),
                    _ => panic!("Unexpected element."),
                }
            )+
            Page { name: node, cost: cost }
        }
    };
}

fn main() {
    // 下図は、使用する有向グラフです。
    // ノード番号はさまざまな状態に対応し、
    // エッジの重みは1つのノードから別のノードに移動するコストを表しています。
    // エッジは一方向であることに注意してください。
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // グラフは隣接リストとして表され、ノード値に対応する各インデックスには、
    // 発信エッジのリストがあります。その効率性から選ばれました。

    let json = include!("./../target/converted_data.rs");

    assert_eq!(get_low_cost(&json, "a", "b"), Some(1));
    assert_eq!(get_low_cost(&json, "a", "d"), Some(3));
    assert_eq!(get_low_cost(&json, "d", "a"), Some(7));
    assert_eq!(get_low_cost(&json, "a", "e"), Some(5));
    assert_eq!(get_low_cost(&json, "e", "a"), None);

    get_low_cost(&json, "a", "e");
}
