#[derive(Debug)]
pub struct AccessUnit {
    pub unit_number: String,
    pub name: String,
    pub year_start: i32,
    pub year_end: i32,
    pub pages_count: i32,
    pub exact_date: String,
    pub inventory_number: String,
    pub fund_number: String,
}