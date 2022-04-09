#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use chrono::Utc;
    use duckdb::{params, Connection};

    #[test]
    fn timestamp_appender_minimal_example_sig_segv() {
        let db = Connection::open_in_memory().unwrap();

        let create_table_sql = r"
          CREATE TABLE item (
              id INTEGER NOT NULL,
              ts TIMESTAMP
          );";
        db.execute_batch(create_table_sql).unwrap();

        let mut app = db.appender("item").unwrap();
        let row_count = 10;
        for i in 0..row_count {
            app.append_row(params![i, "1970-01-01T00:00:00Z"]).unwrap();
        }

        let val = db
            .query_row("SELECT count(1) FROM item", [], |row| {
                <(u32,)>::try_from(row)
            })
            .unwrap();

        assert_eq!(val, (row_count,));
    }

    #[test]
    fn timestamp_appender_with_chrono_sig_segv() {
        let db = Connection::open_in_memory().unwrap();
        let now = Utc::now();

        let create_table_sql = r"
          CREATE TABLE item (
              id INTEGER NOT NULL,
              ts TIMESTAMP
          );";
        db.execute_batch(create_table_sql).unwrap();

        let mut app = db.appender("item").unwrap();
        let row_count = 10;
        for i in 0..row_count {
            app.append_row(params![i, now]).unwrap();
        }

        let val = db
            .query_row("SELECT count(1) FROM item", [], |row| {
                <(u32,)>::try_from(row)
            })
            .unwrap();

        assert_eq!(val, (row_count,));
    }

    #[test]
    fn varchar_appender_works() {
        let db = Connection::open_in_memory().unwrap();

        let create_table_sql = r"
          CREATE TABLE item (
              id INTEGER NOT NULL,
              ts VARCHAR
          );";
        db.execute_batch(create_table_sql).unwrap();

        let mut app = db.appender("item").unwrap();
        let row_count = 10;
        for i in 0..row_count {
            app.append_row(params![i, "1970-01-01T00:00:00Z"]).unwrap();
        }

        let val = db
            .query_row("SELECT count(1) FROM item", [], |row| {
                <(u32,)>::try_from(row)
            })
            .unwrap();

        assert_eq!(val, (row_count,));
    }

    #[test]
    fn chrono_works_on_insert() {
        let db = Connection::open_in_memory().unwrap();
        use chrono::Utc;

        let create_table_sql = r"
          CREATE TABLE item (
              id INTEGER NOT NULL,
              ts TIMESTAMP
          );";
        db.execute_batch(create_table_sql).unwrap();

        let row_count = 10;
        let ts = Utc::now();
        for i in 0..row_count {
            db.execute(
                "INSERT INTO item (id, ts) VALUES (?1, ?2)",
                duckdb::params![i, ts],
            )
            .unwrap();
        }

        let val = db
            .query_row("SELECT count(1) FROM item", [], |row| {
                <(u32,)>::try_from(row)
            })
            .unwrap();

        assert_eq!(val, (row_count,));
    }

    #[test]
    fn ts_str_works_on_insert() {
        let db = Connection::open_in_memory().unwrap();

        let create_table_sql = r"
          CREATE TABLE item (
              id INTEGER NOT NULL,
              ts TIMESTAMP
          );";
        db.execute_batch(create_table_sql).unwrap();

        let row_count = 10;
        let ts = "2020-01-01T00:00:00Z";
        for i in 0..row_count {
            db.execute(
                "INSERT INTO item (id, ts) VALUES (?1, ?2)",
                duckdb::params![i, ts],
            )
            .unwrap();
        }

        let val = db
            .query_row("SELECT count(1) FROM item", [], |row| {
                <(u32,)>::try_from(row)
            })
            .unwrap();

        assert_eq!(val, (row_count,));
    }
}
