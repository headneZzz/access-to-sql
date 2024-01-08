use chrono::{DateTime, Utc};
use regex::Regex;
use uuid::Uuid;
use crate::models::access::AccessUnit;

#[derive(Debug)]
pub struct TblUnit {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub creation_date_time: DateTime<Utc>,
    pub status_id: Uuid,
    pub deleted: bool,
    pub isn_unit: i64,
    pub isn_inventory: i64,
    pub isn_doc_type: i64,
    pub isn_securlevel: i64,
    pub security_char: String,
    pub isn_inventory_cls: i64,
    pub unit_kind: i32,
    pub unit_num_1: String,
    pub unit_num_2: Option<String>,
    pub name: String,
    pub is_in_search: String,
    pub is_lost: String,
    pub has_sf: String,
    pub has_fp: String,
    pub has_defects: String,
    pub catalogued: String,
    pub weight: i32,
    pub start_year: i32,
    pub start_year_inexact: String,
    pub end_year: i32,
    pub end_year_inexact: String,
    pub medium_type: String,
    pub has_treasures: String,
    pub is_museum_item: String,
    pub page_count: i32,
    pub cardboarded: String,
    pub all_date: String,
}

impl TblUnit {
    pub fn new(access_unit: &AccessUnit, isn_inventory: i64, isn_unit: i64) -> TblUnit {
        let regex = Regex::new(r"(\d*)(\D*)").unwrap();
        let caps = regex.captures(&access_unit.unit_number).unwrap();

        TblUnit {
            id: Uuid::new_v4(),
            owner_id: Uuid::parse_str("12345678-9012-3456-7890-123456789012").unwrap(),
            creation_date_time: Utc::now(),
            status_id: Uuid::parse_str("DD6ABDFF-D922-4746-80B8-15BE426E3849").unwrap(),
            deleted: false,
            isn_unit,
            isn_inventory,
            isn_doc_type: 1,
            isn_securlevel: 1,
            security_char: "o".to_string(),
            isn_inventory_cls: isn_inventory,
            unit_kind: 703,
            unit_num_1: caps.get(1).map_or("", |m| m.as_str()).to_string(),
            unit_num_2: caps.get(2).map_or(None, |m| Some(m.as_str().to_string())),
            name: access_unit.name.clone(),
            is_in_search: "N".to_string(),
            is_lost: "N".to_string(),
            has_sf: "N".to_string(),
            has_fp: "N".to_string(),
            has_defects: "N".to_string(),
            catalogued: "N".to_string(),
            weight: 0,
            start_year: access_unit.year_start,
            start_year_inexact: "N".to_string(),
            end_year: access_unit.year_end,
            end_year_inexact: "N".to_string(),
            medium_type: "T".to_string(),
            has_treasures: "N".to_string(),
            is_museum_item: "N".to_string(),
            page_count: access_unit.pages_count,
            cardboarded: "N".to_string(),
            all_date: access_unit.exact_date.clone(),
        }
    }
}