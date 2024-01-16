use std::str::Lines;
use log::{info, warn};
use odbc::Connection;
use odbc::odbc_safe::AutocommitOn;
use crate::db::access::fetch_access_data;
use crate::db::sql_server::{get_isn_inventory, get_max_isn_unit, get_non_existent_units, insert_inventory_structure, insert_new_units, is_inventory_structure_exists_by_isn_inventory_cls};
use crate::models::sql_server::TblUnit;

pub fn process(lines: Lines, access_conn: Connection<AutocommitOn>, sql_server_conn: Connection<AutocommitOn>) {
    let mut total_units: usize = 0;
    let mut inserted_units: usize = 0;
    for line in lines {
        let mut iter = line.split_whitespace();

        let fund_num = iter.next().expect("Не найден фонд в текстовом файле");
        let inventory_num = iter.next().expect("Не найдена опись в текстовом файле");
        let mut isn_unit = get_max_isn_unit(&sql_server_conn).expect("Ошибка при получении isn_unit из MS SQL Server");
        let isn_inventory = get_isn_inventory(&sql_server_conn, fund_num, inventory_num).expect("Ошибка при получении isn_inventory из MS SQL Server");

        info!("=== Фонд {} опись {} ===", fund_num, inventory_num);

        if isn_inventory == -1 {
            warn!("Не найден isn_inventory! Пропускаем опись.");
            continue;
        }

        let access_units = fetch_access_data(&access_conn, fund_num, inventory_num).expect("Ошибка при получении дел из MS Access");
        let units: Vec<TblUnit> = access_units.iter().map(|access_unit| {
            isn_unit += 1;
            TblUnit::new(access_unit, isn_inventory, isn_unit)
        }).collect();
        total_units += units.len();

        info!("Найдело дел: {}", units.len());

        if !is_inventory_structure_exists_by_isn_inventory_cls(&sql_server_conn, isn_inventory).unwrap() {
            insert_inventory_structure(&sql_server_conn, isn_inventory).unwrap()
        }

        let non_existent_units = get_non_existent_units(&sql_server_conn, fund_num, inventory_num, &units).expect("Ошибка при получении не перенесенных дел в MS SQL Server");

        if !non_existent_units.is_empty() {
            info!("Дел, которых ещё нет в MS SQL Server: {}", non_existent_units.len());
            inserted_units += non_existent_units.len();
            insert_new_units(&sql_server_conn, non_existent_units).expect("Ошибка при переносе новых дел в MS SQL Server");
        } else {
            info!("Все найденные дела этой описи уже существуют в MS SQL Server");
        }
    }
    info!("Всего найдено дел в MS Access: {}", total_units);
    info!("Всего дел, которых не было в MS SQL Server: {}", inserted_units);
}