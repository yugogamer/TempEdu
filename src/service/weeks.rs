use std::ops::Add;
use actix_web::error;
use chrono::{DateTime, Local};
use thiserror::Error;
use tokio_postgres::Client;


#[derive(Debug, Error)]
pub enum WeeksError{
    #[error("Error api : mapper")]
    MapperError(#[from] tokio_pg_mapper::Error),
    #[error("Error api : database : `{0}`")]
    DbError(#[from] tokio_postgres::Error),
}

pub async fn create_weeks(conn: &Client, iso_string : String) -> Result<(), WeeksError>{ 
    let (year, week, _week, start_date, end_date) = generate_weeks(iso_string);

    let rows = conn.query("
    INSERT INTO weeks (week, year, start_time, end_time)
    VALUES ($1, $2, $3, $4, $5)
    ", &[&week, &year, &start_date, &end_date]).await;

    match rows {
        Ok(_) => Ok(()),
        Err(e) => Err(WeeksError::DbError(e)),
    }
}

pub fn generate_weeks(iso_string : String) -> (i32, i32, i32, DateTime<Local>, DateTime<Local>){
    let date: DateTime<Local> = iso_string.parse::<chrono::DateTime<Local>>().unwrap();
    let end_date = date.add(chrono::Duration::days(6).add(chrono::Duration::hours(23).add(chrono::Duration::minutes(59))));
    let year = date.format("%Y").to_string().parse::<i32>().unwrap();
    let week = date.format("%V").to_string().parse::<i32>().unwrap();
    let week_end = end_date.format("%V").to_string().parse::<i32>().unwrap();

    return (year, week, week_end, date, end_date);
}

#[cfg(test)]
mod test{
    use chrono::{Local, DateTime, TimeZone};

    use super::generate_weeks;


    #[test]
    fn test_date(){
        let iso_string = "2022-04-03T22:00:00.000Z".to_owned();
        let (year, week, week_end, date, end_date) = generate_weeks(iso_string);
        let lundi:DateTime<Local> = Local.ymd(2022, 4, 4).and_hms(0, 0, 0);
        let dimanche:DateTime<Local> = Local.ymd(2022, 4, 10).and_hms(23, 59, 0);
        assert_eq!(year, 2022);
        assert_eq!(week, 14);
        assert_eq!(week_end, 14);
        assert_eq!(date, lundi);
        assert_eq!(end_date, dimanche);
    }
}

impl error::ResponseError for WeeksError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            WeeksError::MapperError(_) => {
                actix_web::HttpResponse::InternalServerError().json(self.to_string())
            },
            WeeksError::DbError(_) => {
                actix_web::HttpResponse::InternalServerError().json(self.to_string())
            },
        }
    }
}
