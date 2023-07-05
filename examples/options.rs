use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: f64,
    //#[serde(skip_serializing, default)] ///提供默认值
    #[serde(skip_serializing, default="default_y")] ///手动设置提供值的函数
    y: f64,
}

///一些选项 {#一些选项}
/// 忽略某个字段 {#忽略某个字段}
/// #[serde(skip_serialize)] 在序列化时忽略该字段
/// #[serde(skip_deserialize)] 在反序列化时忽略该字段
/// #[serde(skip)] 同时忽略这个字段

fn main() {
    let point = Point {
        x: 1.0,
        y: 2.0
    };
    println!("point: {:#?}", point);
    let json: String = serde_json::to_string(&point).unwrap(); 
    println!("{}", json);

    let point: Point = serde_json::from_str(&json).unwrap(); 
    println!("{:#?}", point);
}

fn default_y() -> f64 {
    5.0
}