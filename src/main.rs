extern crate redis;

fn main() {
    let mut conn: redis::Connection = connect();

    let _: () = redis::cmd("SET")
        .arg("heen")
        .arg("deen")
        .query(&mut conn)
        .expect("SET failed");

    let obj: String = redis::cmd("GET")
        .arg("heen")
        .query(&mut conn)
        .expect("GET failed");

    println!("value for 'heen' = {}", obj);
}

fn connect() -> redis::Connection {
    redis::Client::open("redis://127.0.0.1:6379")
        .expect("Invalid connection")
        .get_connection()
        .expect("Failed to connect")
}
