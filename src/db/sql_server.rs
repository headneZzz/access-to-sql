use odbc::{Connection, Data, Statement};
use odbc::odbc_safe::AutocommitOn;
use chrono::{Utc};
use uuid::Uuid;
use crate::models::sql_server::{IsnCodes, TblUnit};

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
                               unit.unit_num_2.unwrap(),
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

    println!("{}", insert_sql);
    let mut stmt = Statement::with_parent(conn)?;
    stmt.exec_direct(&*insert_sql)?;

    Ok(())
}