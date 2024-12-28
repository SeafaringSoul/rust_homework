use serde::Serialize;
use serde_json::Value;

fn main() {
    json_format();
}

/** serde-json 匹配
{
    "name" : "ace",
    "age" : 18,
    "email" : "cew@qq.com",
    "address" :{
        "street" : "地址在哪里"，
        "city" : "beijing",
    },
    "phone_numbers" : ["111-111-11111","222-222-2222"]
}
**/

#[derive(Serialize)]
struct Address{
    street:String,
    city: String,
}

#[derive(Serialize)]
struct Personal{
    name:String,
    age:u32,
    email:String,
    address:Address,
    phone_numbers:Vec<String>,     // 动态长度的字符串数组
}

fn json_format (){
    // 结构体序列化
    let personal = Personal{
        name:"yuFu".to_string(),
        age:22,
        email:"111222@gmail.com".to_string(),
        address: Address{
            street:"地址".to_string(),
            city:"北京".to_string(),
        },
        phone_numbers:vec!["111-111-1111".to_string(),"222-222-2222".to_string()]
    };

    let json_str = serde_json::to_string(&personal).unwrap();
    println!("{}", json_str);

    // 提取json对象不同的字段
    let json_data: Value = serde_json::from_str(&json_str).expect("JSON is error");

    if let Value::Object(obj) = &json_data {
        if let Some(Value::String(name)) = obj.get("name") {
            println!("name: {}", name);
        }
        if let Some(Value::Number(age)) = obj.get("age") {
            if let Some(age) = age.as_u64(){
                println!("age: {}", age);
            }
        }
        if let Some(Value::String(email)) = obj.get("email") {
            println!("Email: {}", email);
        }
        if let Some(Value::Object(address)) = obj.get("address") {
            if let Some(Value::String(street)) = address.get("street") {
                println!("street: {}", street);
            }
            if let Some(Value::String(city)) = address.get("city") {
                println!("city: {}", city);
            }
        }

        if let Some(Value::Array(phone_numbers)) = obj.get("phone_numbers") { // 键名修正为 phone_numbers
            for (i, v) in phone_numbers.iter().enumerate() {
                if let Value::String(phone) = v { // 解构为字符串
                    println!("电话 {}: {}", i + 1, phone); // 打印电话信息
                }
            }
        }

    }else{
        println!("解析失败！");
    }
}


