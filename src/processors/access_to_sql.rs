use std::str::Lines;
use log::info;
use odbc::Connection;
use odbc::odbc_safe::AutocommitOn;
use crate::db::access::fetch_access_data;
use crate::db::sql_server::{get_isn_inventory, get_max_isn_unit, insert_inventory_structure, insert_new_units, is_inventory_structure_exists_by_isn_inventory_cls};
use crate::models::sql_server::TblUnit;

pub fn process(lines: Lines, access_conn: Connection<AutocommitOn>, sql_server_conn: Connection<AutocommitOn>) {
    let mut total_units: usize = 0;
    for line in lines {
        let mut iter = line.split_whitespace();
        let fund_num = iter.next().expect("Не найден фонд в текстовом файле");
        let inventory_num = iter.next().expect("Не найдена опись в текстовом файле");
        let mut isn_unit = get_max_isn_unit(&sql_server_conn).expect("Ошибка при получении isn_unit из MS SQL Server");
        let isn_inventory = get_isn_inventory(&sql_server_conn, fund_num, inventory_num).expect("Ошибка при получении isn_inventory из MS SQL Server");
        if isn_inventory == -1 {
            info!("Для фонда {} описи {} не найден isn_inventory", fund_num, inventory_num);
            continue;
        }

        let access_units = fetch_access_data(&access_conn, fund_num, inventory_num).expect("Ошибка при получении дел из MS Access");
        let units: Vec<TblUnit> = access_units.iter().map(|access_unit| {
            isn_unit += 1;
            TblUnit::new(access_unit, isn_inventory, isn_unit)
        }).collect();
        info!("Фонд {} опись {} найдело дел: {}", fund_num, inventory_num, units.len());
        total_units += units.len();
        if !is_inventory_structure_exists_by_isn_inventory_cls(&sql_server_conn, isn_inventory).unwrap() {
            insert_inventory_structure(&sql_server_conn, isn_inventory).unwrap()
        }
        insert_new_units(&sql_server_conn, units).expect("Ошибка при вставке новых дел в MS SQL Server");
    }
    info!("Всего найдено дел: {}", total_units);
}