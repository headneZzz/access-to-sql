use odbc::{Connection, Data, ResultSetState, Statement};
use odbc::odbc_safe::AutocommitOn;
use chrono::{Utc};
use log::{debug};
use regex::Regex;
use ResultSetState::NoData;
use uuid::Uuid;
pub use crate::models::sql_server::TblUnit;

pub fn get_isn_inventory(conn: &Connection<AutocommitOn>, fund_num: &str, inventory_num: &str) -> odbc::Result<i64> {
    let regex = Regex::new(r"(\d*)(\D*)").unwrap();
    let caps = regex.captures(inventory_num).unwrap();
    let inventory_num_1 = caps.get(1).map_or("", |m| m.as_str()).to_string();
    let inventory_num_2 = caps.get(2).map_or(None, |m| Some(m.as_str().to_string()));
    let mut sql_text = r#"
        SELECT tblINVENTORY.ISN_INVENTORY
        FROM tblINVENTORY
        INNER JOIN tblFUND ON tblINVENTORY.ISN_FUND = tblFUND.ISN_FUND
        WHERE IsNull(NULLIF(tblFUND.FUND_NUM_1, '') + '-', '') + tblFUND.FUND_NUM_2 + IsNull(tblFUND.FUND_NUM_3, '') = ?
        AND tblINVENTORY.INVENTORY_NUM_1 = ?
    "#.to_string();

    let mut stmt = Statement::with_parent(&conn)?;
    stmt = stmt.bind_parameter(1, &fund_num)?;
    stmt = stmt.bind_parameter(2, &inventory_num_1)?;

    if inventory_num_2.is_some() && !inventory_num_2.clone().unwrap().is_empty() {
        sql_text += " AND tblINVENTORY.INVENTORY_NUM_2 = ?";
        stmt = stmt.bind_parameter(3, &inventory_num_2)?;
    }

    debug!("{}", sql_text);

    let mut isn_inventory: i64 = -1;
    if let Data(mut cursor) = stmt.exec_direct(&*sql_text)? {
        if let Some(mut row) = cursor.fetch()? {
            isn_inventory = row.get_data(1)?.unwrap_or_default();
        }
    }

    Ok(isn_inventory)
}

pub fn get_max_isn_unit(conn: &Connection<AutocommitOn>) -> odbc::Result<i64> {
    let sql_text = "SELECT MAX(ISN_UNIT) FROM TblUnit";
    let stmt = Statement::with_parent(&conn)?;
    let mut isn_unit: i64 = 0;
    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        if let Some(mut row) = cursor.fetch()? {
            isn_unit = row.get_data(1)?.unwrap_or_default();
        }
    }

    Ok(isn_unit)
}

pub fn is_inventory_structure_exists_by_isn_inventory_cls(conn: &Connection<AutocommitOn>, isn_inventory_cls: i64) -> odbc::Result<bool> {
    let sql_text = "SELECT ISN_INVENTORY_CLS FROM tblINVENTORY_STRUCTURE WHERE ISN_INVENTORY_CLS = ?";
    let mut stmt = Statement::with_parent(&conn)?;
    stmt = stmt.bind_parameter(1, &isn_inventory_cls)?;
    let mut exists = false;
    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        while let Some(_row) = cursor.fetch()? {
            exists = true;
        }
    }

    Ok(exists)
}

fn create_uuid(most_sig_bits: i64, least_sig_bits: i64) -> Uuid {
    let most_sig_bytes = most_sig_bits.to_be_bytes();
    let least_sig_bytes = least_sig_bits.to_be_bytes();

    let mut uuid_bytes = [0u8; 16];
    uuid_bytes[..8].copy_from_slice(&most_sig_bytes);
    uuid_bytes[8..].copy_from_slice(&least_sig_bytes);

    Uuid::from_bytes(uuid_bytes)
}

pub fn insert_inventory_structure(conn: &Connection<AutocommitOn>, isn_inventory_cls: i64) -> odbc::Result<()> {
    let uuid = create_uuid(0, isn_inventory_cls);
    let insert_sql = &format!(r#"
        INSERT INTO tblINVENTORY_STRUCTURE (ID, OwnerID, CreationDateTime, DocID, RowID, StatusID, Deleted, ISN_INVENTORY_CLS, ISN_INVENTORY, NAME, FOREST_ELEM, PROTECTED, WEIGHT)
        VALUES ('{}', '{}', '{}', '{}', {}, '{}', '{}', {}, {}, '{}', '{}', '{}', {})"#,
                              uuid,
                              "12345678-9012-3456-7890-123456789012",
                              Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
                              "00000000-0000-0000-0000-000000000000",
                              0,
                              "A4366C7E-ACBA-4A71-B59A-15D51107DBFE",
                              false,
                              isn_inventory_cls,
                              isn_inventory_cls,
                              "...",
                              "T",
                              "N",
                              0
    );
    debug!("{}", insert_sql);

    let stmt = Statement::with_parent(conn)?;
    stmt.exec_direct(&*insert_sql)?;

    Ok(())
}

pub fn insert_new_units(conn: &Connection<AutocommitOn>, units: Vec<&TblUnit>) -> odbc::Result<()> {
    let batch_size = 1000;

    for chunk in units.chunks(batch_size) {
        let mut first = true;
        let mut insert_sql = r#"
            INSERT INTO tblUNIT
            (ID, OwnerID, CreationDateTime, StatusID, Deleted, ISN_UNIT, ISN_INVENTORY,
            ISN_DOC_TYPE, ISN_SECURLEVEL, SECURITY_CHAR, ISN_INVENTORY_CLS,
            UNIT_KIND, UNIT_NUM_1, UNIT_NUM_2, NAME, IS_IN_SEARCH, IS_LOST, HAS_SF, HAS_FP,
            HAS_DEFECTS, CATALOGUED, WEIGHT, START_YEAR, START_YEAR_INEXACT, END_YEAR,
            END_YEAR_INEXACT, MEDIUM_TYPE, HAS_TREASURES, IS_MUSEUM_ITEM, PAGE_COUNT,
            CARDBOARDED, ALL_DATE)
            VALUES
        "#.to_string();

        for unit in chunk {
            if !first {
                insert_sql += ", ";
            }
            first = false;
            insert_sql += &format!("('{}', '{}', '{}', '{}', '{}', {}, {}, {}, {}, '{}', {}, {}, {}, '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, {}, '{}', {}, '{}', '{}', '{}', '{}', {}, '{}', '{}')",
                                   unit.id,
                                   unit.owner_id,
                                   unit.creation_date_time.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
                                   unit.status_id,
                                   unit.deleted,
                                   unit.isn_unit,
                                   unit.isn_inventory,
                                   unit.isn_doc_type,
                                   unit.isn_securlevel,
                                   unit.security_char,
                                   unit.isn_inventory_cls,
                                   unit.unit_kind,
                                   unit.unit_num_1,
                                   unit.unit_num_2.clone().unwrap(),
                                   unit.name,
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

        debug!("{}", insert_sql);
        let stmt = Statement::with_parent(conn)?;
        stmt.exec_direct(&insert_sql)?;
    }

    Ok(())
}

pub fn get_non_existent_units<'a>(conn: &Connection<AutocommitOn>, fund_num: &str, inventory_num: &str, units: &'a Vec<TblUnit>) -> odbc::Result<Vec<&'a TblUnit>> {
    let regex = Regex::new(r"(\d*)(\D*)").expect("Failed to create regex");
    let caps = regex.captures(inventory_num).expect("Failed to capture inventory_num with regex");
    let inventory_num_1 = caps.get(1).map_or("", |m| m.as_str()).to_string();
    let inventory_num_2 = caps.get(2).map_or("", |m| m.as_str()).to_string();
    let mut res: Vec<&TblUnit> = Vec::new();

    units.iter()
        .filter(|unit| !unit.unit_num_1.is_empty())
        .for_each(|unit| {
            let mut sql_text = r#"
                SELECT u.UNIT_NUM_1, u.UNIT_NUM_2
                FROM [dbo].[tblUNIT] u
                    INNER JOIN [dbo].[tblINVENTORY] inv ON u.ISN_INVENTORY = inv.ISN_INVENTORY
                    INNER JOIN [dbo].[tblFUND] fund ON fund.ISN_FUND = inv.ISN_FUND
                WHERE IsNull(NULLIF(fund.FUND_NUM_1, '') + '-', '') + fund.FUND_NUM_2 + IsNull(fund.FUND_NUM_3, '') = ?
                    AND inv.INVENTORY_NUM_1 = ?
                    AND u.UNIT_NUM_1 = ?
            "#.to_string();

            let mut stmt = Statement::with_parent(&conn).expect("Failed to create statement with parent");
            stmt = stmt.bind_parameter(1, &fund_num).unwrap();
            stmt = stmt.bind_parameter(2, &inventory_num_1).unwrap();
            stmt = stmt.bind_parameter(3, &unit.unit_num_1).unwrap();
            let mut index = 3;

            if inventory_num_2.is_empty() {
                sql_text += " AND (inv.INVENTORY_NUM_2 IS NULL OR inv.INVENTORY_NUM_2 = '')";
            } else {
                sql_text += " AND inv.INVENTORY_NUM_2 = ?";
                index += 1;
                stmt = stmt.bind_parameter(index, &inventory_num_2).unwrap();
            }

            if unit.unit_num_2.as_ref().expect("unit_num_2 is null").is_empty() {
                sql_text += " AND (u.UNIT_NUM_2 IS NULL OR u.UNIT_NUM_2 = '')";
            } else {
                sql_text += " AND u.UNIT_NUM_2 = ?";
                index += 1;
                stmt = stmt.bind_parameter(index, &unit.unit_num_2).unwrap();
            }

            let cursor = stmt.exec_direct(&*sql_text).expect("Failed to execute direct statement");
            match cursor {
                Data(mut cursor) => {
                    match cursor.fetch().unwrap() {
                        Some(_row) => {}
                        None => {
                            res.push(unit);
                        }
                    }
                }
                NoData(_cursor) => {
                    res.push(unit);
                }
            }
        });


    Ok(res)
}
