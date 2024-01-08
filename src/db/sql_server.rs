use odbc::{Connection, create_environment_v3, Data, Statement};
use odbc::odbc_safe::AutocommitOn;

#[derive(Debug)]
pub struct SqlServerData {
    isn_fund: String,
    isn_inventory: String,
}

pub fn fetch_sql_server_data(conn: &Connection<AutocommitOn>, fund_num: &str, inventory_num: &str) -> odbc::Result<SqlServerData> {
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
    let mut result_set: SqlServerData = SqlServerData { isn_fund: "".to_string(), isn_inventory: "".to_string() };
    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        while let Some(mut row) = cursor.fetch()? {
            let isn_fund: String = row.get_data(1)?.unwrap_or_default();
            let isn_inventory: String = row.get_data(2)?.unwrap_or_default();

            result_set = (SqlServerData {
                isn_fund,
                isn_inventory,
            });
        }
    }

    Ok(result_set)
}