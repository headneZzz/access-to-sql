use odbc::{Connection, create_environment_v3_with_os_db_encoding, Data, Statement};
use odbc::odbc_safe::AutocommitOn;

#[derive(Debug)]
pub struct AccessData {
    номер_дела: String,
    наименование_дела: String,
    год_начала: i32,
    год_конца: i32,
    количество_листов: i32,
    точная_дата: String,
    номер_описи: String,
    номер_фонда: String,
}

pub fn fetch_access_data(conn: &Connection<AutocommitOn>, fund_num: &str, inventory_num: &str) -> odbc::Result<Vec<AccessData>> {
    let sql_text = r#"SELECT
    [Номер_Дела], [Наименование_дела], [Т_Дело].[Год_начала], [Т_Дело].[Год_конца], [Кол-во_листов], [Точная_дата], [Т_Описи].[Номер_Описи], [Т_Фонд].[Номер_фонда]
    FROM
    ([Т_Дело]
    INNER JOIN [Т_Описи] ON [Т_Дело].[Код_Описи] = [Т_Описи].[Код_Описи])
    INNER JOIN [Т_Фонд] ON [Т_Описи].[Код_фонда] = [Т_Фонд].[Код_фонда]
    WHERE [Т_Фонд].[Номер_фонда] = ? AND [Т_Описи].[Номер_Описи] = ?
    "#;
    let mut stmt = Statement::with_parent(conn)?;
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


