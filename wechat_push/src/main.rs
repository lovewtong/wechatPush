use std::collections::HashMap;
use chrono::{Local, NaiveDate, Duration};
use reqwest::Client;
use serde::{de::value, Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

// 定义天气
#[derive(Deserialize, Debug)]
struct Weather {
    night_wind_direction: String, // 夜间风向
    date: String, // 具体日期
    high_temperature: String, // 最高温度
    week_day: String, // 星期
    night_weather_text: String, // 夜间天气描述
    day_wind_direction: String, // 白天风向
    low_temperature: String, // 最低温度
    night_air_quality_index: String, // 夜间空气质量指数
    day_weather_text: String, // 白天天气描述
    day_air_quality_index: String, // 白天空气质量指数
    current_weather_text: String, // 当前天气描述
    current_temperature: String,
    wind_speed: String, // 风速等级
    wind_direction: String,
}

// 定义模板数据
#[derive(Serialize, Debug)]
struct TemplateData{
    value: String,
    color: String,
}

// 定义消息模板
#[derive(Serialize, Debug)]
struct TemplateMessage{
    to_user: String,
    template_id: String,
    data:HashMap<String, TemplateData>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather = get_weather().await?;

    print!("weather is:", weather);
    // push_message(&weather).await?;
    Ok(())
}

// 获取天气情况
async fn get_weather() -> Result<Weather, Box<dyn std::error::Error>> {

    // 加载.env文件
    dotenv().ok();

    // 从.env文件读取变量
    let district_id = env::var("DISTRICT_ID")?;
    let data_type = env::var("DATA_TYPE")?;
    let ak = env:: var("AK")?;

    // 发送
    let client = Client::new();
    let res = client
        .get("https://api.map.baidu.com/weather/v1/")
        .query(&[
            ("district_id", district_id),
            ("date_type", data_type),
            ("ak", ak)
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let forecasts = res["result"]["forecasts"].as_array().unwrap();
    let weather = &forecasts[0]; // 获取今天的预报
    let now = &res["result"]["now"];

    Ok(Weather {
        current_weather_text: now["text"].as_str().unwrap().to_string(),
        current_temperature: now["temp"].as_str().unwrap().to_string(),
        wind_speed: now["wind_class"].as_str().unwrap().to_string(),
        wind_direction: now["wind_dir"].as_str().unwrap().to_string(),
        ..serde_json::from_value(weather.clone())?
    })            
}
 
async fn push_message(weather: &Weather) -> Result<(), Box<dyn std::error::Error>> {

    let love_days = calculate_love_days("2016-02-07");
    let birthday_tong = calculate_birthday("2000-07-09");
    let birthday_sy = calculate_birthday("2000-01-06");

    let mut data = HashMap::new();
    
    data.insert("dates", TemplateData {value:format!("{} {}", weather.date, weather.week_day), color: "#00BFFF".to_string() });
    data.insert("climatic", TemplateData {value: weather.current_weather_text.clone(), color: "#00FFF".to_string() });
    data.insert("mini_temperature", TemplateData {value: weather.low_temperature.clone(), cloro: "00FFF".to_string() });
    data.insert("high_temperature", TemplateData {value: weather.high_temperature.clone(), cloro: "00FFF".to_string() });
    data.insert("current_temperature", TemplateData {value: weather.current_temperature.clone(), cloro: "00FFF".to_string() });
    data.insert("wind_speed", TemplateData {value: weather.wind_speed.clone(), cloro: "00FFF".to_string() });
    data.insert("wind_direction", TemplateData {value: weather.wind_direction.clone(), cloro: "00FFF".to_string() });
    // data.insert()

    let commemoration_day = if love_days % 365 == 0{
        format!("今天是恋爱{}周年纪念日!", love_days / 365)
    } else if birthday_tong == 0 {
        "宝贝大人,生日快乐！".to_string()
    } else if birthday_sy == 0{
        "猪猪生日到咯".to_string()
    } else {
        "喜欢桐宝的每一天".to_string()
    };
    data.insert("commemoration_day", TemplateData {value: commemoration_day, color: "#00FFF".to_string() });

    let template_message = TemplateMessage {
        to_user: "ogOY963T99G1hcdFiNvH0L1vsZUo".to_string(),
        template_id: "eiQQIjgSHokSM27vst9MEbz10tpoVtPlkuzqpMvlBwE".to_string(),
        data,
    };

    // 获取access_token
    let client = Client::new();
    let access_token = "";
    let res = client
                .post(format!(
                    "https://api.weixin.qq.com/cgi-bin/message/template/send?access_token={}",
            access_token
                ))
                .json(&template_message)
                .send()
                .await?;
        
        println!("{:?}", res);
        Ok(())
}


// 计算恋爱天数
fn calculate_love_days(start_date: &str) -> i64 {
    let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let today = Local::now().date_naive();
    let duration = today.signed_duration_since(start_date);
    duration.num_days()
}

// 计算距离生日的天数
fn calculate_birthday(birthday_str: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let birthday = NaiveDate::parse_from_str(birthday_str, "%Y-%m-%d")?;
    let today = Local::now().date_naive();
    let next_birthday = if birthday.month() < today.month() || (birthday.month() == today.month() && birthday.day() < today.day()) {
        birthday.with_year(today.year() + 1).unwrap()
    } else {
        birthday.with_year(today.year()).unwrap()
    };
    let duration = next_birthday.signed_duration_since(today);
    Ok(duration.num_days())
}