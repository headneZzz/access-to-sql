mod fetch_access;

use odbc::*;

fn main() {
    // Инициализация логгера
    env_logger::init();

    // Получение данных из MS Access
    match fetch_access_data("Р-1", "1") {
        Ok(access_data) => {
            println!("Данные MS Access получены успешно.");

            // Получение данных из MS SQL Server
            match fetch_sql_server_data("Р-1", "1") {
                Ok(sql_server_data) => {
                    println!("Данные MS SQL Server получены успешно.");

                    // Объединение и обработка данных
                    // Здесь должна быть ваша логика объединения данных
                    // Например, вы можете итерировать по массивам данных и сравнивать или объединять их
                    // Ниже просто печатаем полученные данные
                    for data in access_data {
                        println!("Access Data: {:?}", data);
                    }
                    for data in sql_server_data {
                        println!("SQL Server Data: {:?}", data);
                    }
                }
                Err(e) => println!("Ошибка при получении данных из MS SQL Server: {:?}", e),
            }
        }
        Err(e) => println!("Ошибка при получении данных из MS Access: {:?}", e),
    }
}

fn fetch_sql_server_data(fund_num: &str, inventory_num: &str) -> Result<Vec<SqlServerData>> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;
    let conn_str = "Driver={SQL Server};Server=localhost;Database=tag1;Uid=sa;Pwd=Password_11";
    let conn = env.connect_with_connection_string(&conn_str)?;
    let sql_text = r#"
    select tblFUND.ISN_FUND, tblINVENTORY.ISN_INVENTORY
    from tblINVENTORY
    inner join tblFUND on tblINVENTORY.ISN_FUND = tblFUND.ISN_FUND
    "#;

    let mut stmt = Statement::with_parent(&conn)?;
    let mut result_set: Vec<SqlServerData> = vec![];

    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        while let Some(mut row) = cursor.fetch()? {
            let isn_fund: String = row.get_data(1)?.unwrap_or_default();
            let isn_inventory: String = row.get_data(2)?.unwrap_or_default();

            result_set.push(SqlServerData {
                isn_fund,
                isn_inventory,
            });
        }
    }

    Ok(result_set)
}


fn fetch_access_data(fund_num: &str, inventory_num: &str) -> Result<Vec<AccessData>> {
    let mut env = create_environment_v3_with_os_db_encoding("windows-1251", "windows-1251").unwrap();
    let conn_str = "Driver={Microsoft Access Driver (*.mdb, *.accdb)};DBQ=D:/test/test.mdb;";
    let conn = env.connect_with_connection_string(conn_str)?;

    let sql_text = r#"SELECT
    [Номер_Дела], [Наименование_дела], [Т_Дело].[Год_начала], [Т_Дело].[Год_конца], [Кол-во_листов], [Точная_дата], [Т_Описи].[Номер_Описи], [Т_Фонд].[Номер_фонда]
    FROM
    ([Т_Дело]
    INNER JOIN [Т_Описи] ON [Т_Дело].[Код_Описи] = [Т_Описи].[Код_Описи])
    INNER JOIN [Т_Фонд] ON [Т_Описи].[Код_фонда] = [Т_Фонд].[Код_фонда]
    WHERE [Т_Фонд].[Номер_фонда] = ? AND [Т_Описи].[Номер_Описи] = ?
    "#;
    let mut stmt = Statement::with_parent(&conn)?;
    stmt = stmt.bind_parameter(1, &fund_num)?;
    stmt = stmt.bind_parameter(2, &inventory_num)?;
    let mut result_set = vec![];

    if let Data(mut cursor) = stmt.exec_direct(sql_text)? {
        while let Some(mut row) = cursor.fetch()? {
            // Извлечение данных из каждого столбца
            let номер_дела: String = row.get_data(1)?.unwrap_or_default();
            let наименование_дела: String = row.get_data(2)?.unwrap_or_default();
            let год_начала: i32 = row.get_data(3)?.unwrap_or_default();
            let год_конца: i32 = row.get_data(4)?.unwrap_or_default();
            let количество_листов: i32 = row.get_data(5)?.unwrap_or_default();
            let точная_дата: String = row.get_data(6)?.unwrap_or_default();
            let номер_описи: String = row.get_data(7)?.unwrap_or_default();
            let номер_фонда: String = row.get_data(8)?.unwrap_or_default();

            // Создание структуры данных для хранения извлеченных данных
            let data = AccessData {
                номер_дела,
                наименование_дела,
                год_начала,
                год_конца,
                количество_листов,
                точная_дата,
                номер_описи,
                номер_фонда,
            };
            result_set.push(data);
        }
    }
    Ok(result_set)
}

#[derive(Debug)]
struct AccessData {
    номер_дела: String,
    наименование_дела: String,
    год_начала: i32,
    год_конца: i32,
    количество_листов: i32,
    точная_дата: String,
    номер_описи: String,
    номер_фонда: String,
}

#[derive(Debug)]
struct SqlServerData {
    isn_fund: String,
    isn_inventory: String,
}