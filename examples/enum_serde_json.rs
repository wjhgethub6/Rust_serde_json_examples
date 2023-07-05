use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
///添加一个 serde(tag) 宏
#[serde(tag = "WeekDay")]
enum Week {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "ip")]
enum IP {
    IPv4(String),
    IPv6(String)
}
///枚举的序列化与反序列化
fn main() {
    println!("第一种枚举类型:");
    let json: String = serde_json::to_string(&Week::Friday).unwrap();
    println!("{}", json);

    let week: Week = serde_json::from_str(&json).unwrap();
    println!("{:#?}", week);

    println!("");
    println!("第二种枚举类型:");
    let json: String = serde_json::to_string(&IP::IPv4("127.0.0.1".to_string())).unwrap();
    println!("{}", json);

    let ip: IP = serde_json::from_str(&json).unwrap();
    println!("{:#?}", ip);
    
}