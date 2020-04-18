use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
}

// 優先キューは「Ord」に依存します。
// キューが最小ヒープになるように、特性を明示的に実装する
// 最大ヒープの代わりに。
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // コストの順序を入れ替えていることに注意してください。
        // 引き分けの場合は、位置を比較します。
        // このステップは、 `PartialEq`と` Ord`の実装を一貫させるために必要です。
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd`も実装する必要があります。
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// 「start」から開始し、「dist」を使用して各ノードまでの現在の最短距離を追跡します。
// この実装は、重複したノードをキューに残す可能性があるため、メモリ効率が良くありません。
// また、より単純な実装のために、センチネル値として「usize :: MAX」を使用します。
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<State> {

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // コストゼロで、 `start`にいます。
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // 最初に低コストのノードでフロンティアを調べます（最小ヒープ）
    while let Some(State { cost, position }) = heap.pop() {
        println!("total_cost: {}, position: {}", cost, position);
        // 最短経路を見つけることができたら終了
        if position == goal { return Some(State { cost: cost, position: position }); }

        // すでにより良い方法を見つけていたらスキップ
        if cost > dist[position] { continue; }

        // 到達できる各ノードについて、このノードを通過するコストがより低い方法を見つけることができるかどうかを確認
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // もしそうなら、それをフロンティアに追加して続行してください
            if next.cost < dist[next.position] {
                heap.push(next);
                // 休息、私たちは今より良い方法を見つけました
                dist[next.position] = next.cost;
            }
        }
    }

    // 目標に到達できなかった場合
    None
}


// ===== ===== ===== ===== ===== ===== ===== ===== ===== =====


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Page<'a> {
    name: &'a str,
    cost: u8,
}

// 優先キューは「Ord」に依存します。
// キューが最小ヒープになるように、特性を明示的に実装する
// 最大ヒープの代わりに。
impl Ord for Page<'_> {
    fn cmp(&self, other: &Page) -> Ordering {
        // コストの順序を入れ替えていることに注意してください。
        // 引き分けの場合は、位置を比較します。
        // このステップは、 `PartialEq`と` Ord`の実装を一貫させるために必要です。
        other.cost.cmp(&self.cost).then_with(|| self.name.cmp(&other.name))
    }
}

// `PartialOrd`も実装する必要があります。
impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Page) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_low_cost<'page>(
    adjacent_list: &'page std::collections::HashMap<&str, std::vec::Vec<Page<'_>>>,
    start: &'page str,
    goal: &str
) -> Option<Page<'page>> {

    let mut distance: Vec<_> = (0..adjacent_list.len()).map(|_| usize::MAX).collect();

    let mut hash: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    
    let mut vec = Vec::new();
    for (i, key) in adjacent_list.keys().enumerate() {
        // println!("{}, {}", i, key);
        hash.insert(key, i);
        vec.push(key);
    }

    let start_position: usize = hash[start];
    // let goal_position: usize = hash[goal];
    distance[start_position] = 0;

    println!("{:?}", distance);

    let mut heap = BinaryHeap::new();
    heap.push(Page { cost: 0, name: start });

    while let Some(Page { cost, name }) = heap.pop() {
        println!("total_cost: {}, name: {}", cost, name);

        if name == goal { return Some(Page { cost: cost, name: name }); }

        if cost > distance[hash[name]] as u8 { continue; }

        // println!("{:?}, {:?}, {:?}", vec, vec[position], position);
        for edge in &adjacent_list[name] {
            let next = Page { cost: cost + edge.cost, name: edge.name };
            let dis: usize = hash[next.name];
            if next.cost < distance[dis] as u8 {
                heap.push(next);
                distance[dis] = next.cost as usize;
            }
        }
    }
    None
}

// error example
// macro_rules! json_parse {
//     ({
//         $($key:tt : [
//             $( {
//                 $( $state_key:tt : $state_value:expr ),+
//             } ),*
//         ]),+
//     }) => {
//         {
//             let mut dictionary = ::std::collections::HashMap::new();
//             $(
//                 let mut page = Vec::new();
//                 $(
//                     let mut node: &str = "";
//                     let mut cost: u32 = 0;
//                     $(
//                         match $state_key {
//                             "node" => node = $state_value,
//                             "cost" => cost = $state_value,
//                             _ => panic!("Unexpected element."),
//                         }
//                     )+
//                     let state = State { node: node, cost: cost };
//                     page.push(state);
//                 )*
//                 dictionary.insert($key, page);
//             )+
//             dictionary
//         }
//     };
// }

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
            let mut cost: u8 = 0;
            $(
                match $key {
                    "node" => node = $value,
                    "cost" => cost = $value.parse::<u8>().unwrap(),
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

    let graph = vec![
        // Node 0
        vec![Edge { node: 2, cost: 10 },
             Edge { node: 1, cost: 1 }],
        // Node 1
        vec![Edge { node: 3, cost: 2 }],
        // Node 2
        vec![Edge { node: 1, cost: 1 },
             Edge { node: 3, cost: 3 },
             Edge { node: 4, cost: 1 }],
        // Node 3
        vec![Edge { node: 0, cost: 7 },
             Edge { node: 4, cost: 2 }],
        // Node 4
        vec![]];

    // assert_eq!(shortest_path(&graph, 0, 1), Some(1));
    // assert_eq!(shortest_path(&graph, 0, 3), Some(3));
    // assert_eq!(shortest_path(&graph, 3, 0), Some(7));
    // assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    // assert_eq!(shortest_path(&graph, 4, 0), None);

    println!("{:?}", shortest_path(&graph, 0, 4).unwrap());

    let json = json_parse!({
        "a": [
            { "node": "c", "cost": "10" },
            { "node": "b", "cost": "1" }
        ],
        "b": [
            { "node": "d", "cost": "2" }
        ],
        "c": [
            { "node": "b", "cost": "1" },
            { "node": "d", "cost": "3" },
            { "node": "e", "cost": "1" }
        ],
        "d": [
            { "node": "a", "cost": "7" },
            { "node": "e", "cost": "2" }
        ],
        "e": [ ]
    });
    println!("{:?}", json);
    get_low_cost(&json, "a", "e");
}
