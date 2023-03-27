use redis::{Commands, Connection};

fn set_and_get(con: &mut Connection) -> redis::RedisResult<isize> {
    con.set("key", 42)?;
    let result: String = con.get("key")?;

    assert_eq!(result, "42".to_string());
    Ok(0)
}

/// Scripting with Lua
///
/// 参考: https://redis.io/docs/manual/programmability/eval-intro/
fn eval(con: &mut Connection) -> redis::RedisResult<isize> {

    // Hello, World!
    // 最後の "0" は KEYS引数 の数を表す
    // KEYS引数: キー用の値を格納するために用意された機能
    let result = redis::cmd("EVAL").arg("return 'Hello, World!'").arg("0").query::<String>(con)?;
    assert_eq!(result, "Hello, World!".to_string());

    // 引数の利用
    let result = redis::cmd("EVAL")
        .arg("return ARGV[1]")
        .arg("0").arg("Hello")
        .query::<String>(con)?;
    assert_eq!(result, "Hello".to_string());

    // スクリプトからRedisの操作
    let result = redis::cmd("EVAL")
        .arg("return redis.call('SET', KEYS[1], ARGV[1])")
        .arg("1").arg("piyo").arg("poyo")
        .query::<String>(con)?;
    assert_eq!(result, "OK".to_string());
    let result = redis::cmd("GET").arg("piyo").query::<String>(con)?;
    assert_eq!(result, "poyo".to_string());

    Ok(0)
}

fn main() {
    let client = redis::Client::open("redis://localhost:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    set_and_get(&mut con).unwrap();
    eval(&mut con).unwrap();
}
