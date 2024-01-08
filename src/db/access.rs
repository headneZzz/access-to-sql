use odbc::{Connection, Data, Statement};
use odbc::odbc_safe::AutocommitOn;

#[derive(Debug)]
pub struct AccessUnit {
    pub номер_дела: String,
    pub наименование_дела: String,
    pub год_начала: i32,
    pub год_конца: i32,
    pub количество_листов: i32,
    pub точная_дата: String,
    pub номер_описи: String,
    pub номер_фонда: String,
}

pub fn fetch_access_data(conn: &Connection<AutocommitOn>, fund_num: &str, inventory_num: &str) -> odbc::Result<Vec<AccessUnit>> {
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
            let номер_дела: String = row.get_data(1)?.unwrap_or_default();
            let наименование_дела: String = row.get_data(2)?.unwrap_or_default();
            let год_начала: i32 = row.get_data(3)?.unwrap_or_default();
            let год_конца: i32 = row.get_data(4)?.unwrap_or_default();
            let количество_листов: i32 = row.get_data(5)?.unwrap_or_default();
            let точная_дата: String = row.get_data(6)?.unwrap_or_default();
            let номер_описи: String = row.get_data(7)?.unwrap_or_default();
            let номер_фонда: String = row.get_data(8)?.unwrap_or_default();

            let data = AccessUnit {
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


