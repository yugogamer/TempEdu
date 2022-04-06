use std::ops::Add;
use chrono::Utc;
use tokio_postgres::Client;





pub async fn create_weeks(conn: &Client, iso_string : String){
    let date = iso_string.parse::<chrono::DateTime<Utc>>().unwrap();
    let end_date = date.add(chrono::Duration::days(6).add(chrono::Duration::hours(23)));
    let year = date.format("%Y").to_string().parse::<i32>().unwrap();
    let week = date.format("%V").to_string().parse::<i32>().unwrap();
    let week_end = end_date.format("%V").to_string().parse::<i32>().unwrap();

    println!("{} : {}\nyear : {}\nweek : {} ->  {}", date, end_date, year, week, week_end);
}