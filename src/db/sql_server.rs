use odbc::{Connection, Data, Statement};
use odbc::odbc_safe::AutocommitOn;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use regex::Regex;
use crate::db::access::AccessUnit;

pub fn get_isn_codes(conn: &Connection<AutocommitOn>, fund_num: &str, inventory_num: &str) -> odbc::Result<IsnCodes> {
    let sql_text = r#"
    select tblFUND.ISN_FUND, tblINVENTORY.ISN_INVENTORY
    from tblINVENTORY
    inner join tblFUND on tblINVENTORY.ISN_FUND = tblFUND.ISN_FUND
    where IsNull(NULLIF(tblFUND.FUND_NUM_1, '') + '-', '') + tblFUND.FUND_NUM_2 + IsNull(tblFUND.FUND_NUM_3, '') = ?
    and tblINVENTORY.INVENTORY_NUM_1 = ?
    "#;

    let mut stmt = Statement::with_parent(&conn)?;
    stmt = stmt.bind_parameter(1, &fund_num)?;
    stmt = stmt.bind_parameter(2, &inventory_num)?;
    let mut result_set: IsnCodes = IsnCodes { isn_fund: 0, isn_inventory: 0 };
    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        while let Some(mut row) = cursor.fetch()? {
            let isn_fund: i64 = row.get_data(1)?.unwrap_or_default();
            let isn_inventory: i64 = row.get_data(2)?.unwrap_or_default();

            result_set = (IsnCodes {
                isn_fund,
                isn_inventory,
            });
        }
    }

    Ok(result_set)
}

pub fn get_max_isn_unit(conn: &Connection<AutocommitOn>) -> odbc::Result<i64> {
    let sql_text = "SELECT MAX(ISN_UNIT) from TblUnit";
    let mut stmt = Statement::with_parent(&conn)?;
    let mut isn_unit: i64 = 0;
    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        while let Some(mut row) = cursor.fetch()? {
            isn_unit = row.get_data(1)?.unwrap_or_default();
        }
    }

    Ok(isn_unit)
}

fn is_inventory_structure_exists_by_isn_inventory_cls(conn: &Connection<AutocommitOn>, isn_inventory_cls: i64) -> odbc::Result<bool> {
    let sql_text = "SELECT ISN_INVENTORY_CLS from tblINVENTORY_STRUCTURE where ISN_INVENTORY_CLS = ?";
    let mut stmt = Statement::with_parent(&conn)?;
    stmt = stmt.bind_parameter(1, &isn_inventory_cls)?;
    let mut exists = false;
    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        while let Some(mut row) = cursor.fetch()? {
            exists = true;
        }
    }

    Ok(exists)
}

fn insert_inventory_structure(conn: &Connection<AutocommitOn>, isn_inventory_cls: i64) -> odbc::Result<()> {
    let insert_sql = &format!(r#"
    insert into tblINVENTORY_STRUCTURE (ID, OwnerID, CreationDateTime, DocID, RowID, StatusID, Deleted, ISN_INVENTORY_CLS,
                                    ISN_INVENTORY, NAME,
                                    FOREST_ELEM, PROTECTED, WEIGHT)
                                    values ('{}', '{}', '{}', '{}', {}, '{}', '{}', {},
                                    {}, '{}',
                                    '{}', '{}', {})
    "#, Uuid::from_fields(0, isn_inventory_cls as u16, 0, &[0, 0, 0, 0, 0, 0, 0, 0]), "12345678-9012-3456-7890-123456789012", Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string(), "00000000-0000-0000-0000-000000000000", 0, "A4366C7E-ACBA-4A71-B59A-15D51107DBFE", false, isn_inventory_cls,
                              isn_inventory_cls, "...",
                              "T", "N", 0);

    let mut stmt = Statement::with_parent(conn)?;
    stmt.exec_direct(&*insert_sql)?;

    Ok(())
}

pub fn insert_new_units(conn: &Connection<AutocommitOn>, units: Vec<TblUnit>) -> odbc::Result<()> {
    let mut first = true;
    let mut insert_sql = r#"
    insert into tblUNIT (ID, OwnerID, CreationDateTime, StatusID, Deleted, ISN_UNIT, ISN_INVENTORY,
                     ISN_DOC_TYPE, ISN_SECURLEVEL, SECURITY_CHAR, ISN_INVENTORY_CLS,
                     UNIT_KIND, UNIT_NUM_1, UNIT_NUM_2, NAME,
                     IS_IN_SEARCH, IS_LOST, HAS_SF, HAS_FP,
                     HAS_DEFECTS, CATALOGUED, WEIGHT, START_YEAR, START_YEAR_INEXACT, END_YEAR,
                     END_YEAR_INEXACT, MEDIUM_TYPE, HAS_TREASURES, IS_MUSEUM_ITEM, PAGE_COUNT,
                     CARDBOARDED, ALL_DATE)
    values
    "#.to_string();
    for unit in units {
        if !is_inventory_structure_exists_by_isn_inventory_cls(conn, unit.isn_inventory_cls).unwrap() {
            insert_inventory_structure(conn, unit.isn_inventory_cls).unwrap()
        }
        if !first {
            insert_sql += ", ";
        }
        first = false;
        insert_sql += &format!(
            "('{}', '{}', '{}', '{}', '{}', {}, {},
                     {}, {}, '{}', {},
                     {}, {}, '{}', '{}',
                     '{}', '{}', '{}', '{}',
                     '{}', '{}', {}, {}, '{}', {},
                     '{}', '{}', '{}', '{}', {},
                     '{}', '{}')",
            unit.id, unit.owner_id, unit.creation_date_time.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(), unit.status_id, unit.deleted, unit.isn_unit, unit.isn_inventory,
            unit.isn_doc_type, unit.isn_securlevel, unit.security_char, unit.isn_inventory_cls,
            unit.unit_kind, unit.unit_num_1, unit.unit_num_2.unwrap(), unit.name,
            unit.is_in_search,
            unit.is_lost,
            unit.has_sf,
            unit.has_fp,
            unit.has_defects,
            unit.catalogued,
            unit.weight,
            unit.start_year,
            unit.start_year_inexact,
            unit.end_year,
            unit.end_year_inexact,
            unit.medium_type,
            unit.has_treasures,
            unit.is_museum_item,
            unit.page_count,
            unit.cardboarded,
            unit.all_date
        );
    }

    println!("{}", insert_sql);
    let mut stmt = Statement::with_parent(conn)?;
    stmt.exec_direct(&*insert_sql)?;

    Ok(())
}

#[derive(Debug)]
pub struct IsnCodes {
    pub isn_fund: i64,
    pub isn_inventory: i64,
}

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
    pub fn new(isn_codes: &IsnCodes, access_unit: &AccessUnit, isn_unit: i64) -> TblUnit {
        let regex = Regex::new(r"(\d*)(\D*)").unwrap();
        let caps = regex.captures(&access_unit.номер_дела).unwrap();

        TblUnit {
            id: Uuid::new_v4(),
            owner_id: Uuid::parse_str("12345678-9012-3456-7890-123456789012").unwrap(),
            creation_date_time: Utc::now(),
            status_id: Uuid::parse_str("DD6ABDFF-D922-4746-80B8-15BE426E3849").unwrap(),
            deleted: false,
            isn_unit,
            isn_inventory: isn_codes.isn_inventory,
            isn_doc_type: 1,
            isn_securlevel: 1,
            security_char: "o".to_string(),
            isn_inventory_cls: isn_codes.isn_inventory,
            unit_kind: 703,
            unit_num_1: caps.get(1).map_or("", |m| m.as_str()).to_string(),
            unit_num_2: caps.get(2).map_or(None, |m| Some(m.as_str().to_string())),
            name: access_unit.наименование_дела.clone(),
            is_in_search: "N".to_string(),
            is_lost: "N".to_string(),
            has_sf: "N".to_string(),
            has_fp: "N".to_string(),
            has_defects: "N".to_string(),
            catalogued: "N".to_string(),
            weight: 0,
            start_year: access_unit.год_начала,
            start_year_inexact: "N".to_string(),
            end_year: access_unit.год_конца,
            end_year_inexact: "N".to_string(),
            medium_type: "T".to_string(),
            has_treasures: "N".to_string(),
            is_museum_item: "N".to_string(),
            page_count: access_unit.количество_листов,
            cardboarded: "N".to_string(),
            all_date: access_unit.точная_дата.clone(),
        }
    }
}