use serde::Deserialize;

#[derive(Deserialize)]
pub struct TimeRangeFilter {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub count: Option<i32>,
}
