use gtfs_structures::Gtfs;

fn main() {
    // GTFSファイルの取得先
    let url = "http://www.kotoden.co.jp/publichtm/gtfs/gtfsdata/latest/gtfs_kd.zip";

    // GTFSファイルの取得処理
    let gtfs = Gtfs::from_url(url).unwrap();

    // 取得したGTFSファイルに関する情報を標準出力
    gtfs.print_stats();

    // 停車地名称の表示（停車地名称が空文字の場合を除く）
    for stop in gtfs.stops {
        if stop.1.name != "" {
            println!("{}", stop.1.name);
        }
    }

    // 出発地と到着地の指定
    let from_point = "瓦町";
    let to_point = "八栗";

    // 時刻表の取得
    let mut times = gtfs.trips.iter().filter_map( |trip| {

        // 方向を特定するための変数
        let mut from_stop_sequence = 0;
        let mut to_stop_sequence = 0;

        // 平日の場合のみ時間を取得する
        if trip.1.service_id == "平日" {
            match trip.1.stop_times.iter().filter_map( |stop_time| {

                // 着駅の停車地の順序を更新
                if stop_time.stop.name == to_point {
                    to_stop_sequence = stop_time.stop_sequence;
                }

                // 発駅から車両が出発する時間を取得し、停車地の順序を更新する
                if stop_time.stop.name == from_point {
                    match stop_time.departure_time {
                        Some(departure_time) => {
                            from_stop_sequence = stop_time.stop_sequence;
                            Some(departure_time)
                        },
                        None => None,
                    }
                } else {
                    None
                }
            }).collect::< Vec<u32> >() {

                // 発駅の後に着駅が来る経路の場合に到着時間を返す
                vec if vec.len() > 0 && from_stop_sequence < to_stop_sequence => {

                    // 一回の運転で複数回同じ駅に停車する場合は最後に停車する時刻を返す
                    Some( vec[vec.len()-1] )
                },
                _ => None,
            }
        } else {
            None
        }
    }).collect::< Vec<u32> >();

    // 整形して表示
    times.sort();
    for time in times {
        println!("{0: >02}:{1: >02}:{2: >02}", (time/3600), (time%3600/60), (time%3600%60));
    }
}



