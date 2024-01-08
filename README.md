# access-to-sql
Консольная утилита на Rust для импорта информации о делах из MS Access в MS Sql Server. 

### Перед запуском
Корректно заполнить конфиг файл [config.toml](config.toml).

Заполнить параметры `access_odbc_connection` и `sql_server_odbc_connection` для подключения к базам данных.

Создать текстовый файл с номерами фондов и описей. Путь до файла указывается в  в параметре
`file_path`. Пример содержания файла:
```text
Р-137	1
Р-137	4л
Р-489	1грф
```