use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

///结构体的序列化与反序列化
fn main() {
    let point = Point {x: 1.0, y: 2.0};
    let json: String = serde_json::to_string(&point).unwrap();
    println!("{}", json);

    let point: Point = serde_json::from_str(&json).unwrap();
    println!("{:#?}", point);
}

