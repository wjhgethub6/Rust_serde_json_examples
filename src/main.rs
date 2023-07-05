use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point {x: 1, y: 2};
    let json: String = serde_json::to_string(&point).unwrap();
    println!("{}", json);

    let point: Point = serde_json::from_str(&json).unwrap();
    println!("{:#?}", point);
}

