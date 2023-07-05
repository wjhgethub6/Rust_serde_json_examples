use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct UnitStruct;

///Unit Struct 的序列化与反序列化 {#unit-struct-的序列化与反序列化}
fn main() {
    let json: String = serde_json::to_string(&UnitStruct).unwrap();
    println!("{}", json);

    let n: UnitStruct = serde_json::from_str(&json).unwrap();
    println!("{:#?}", n);
}