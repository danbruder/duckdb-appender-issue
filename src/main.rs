extern crate duckdb;
use std::convert::TryFrom;

use duckdb::DropBehavior;
use duckdb::{params, Connection, Result};

fn main() -> Result<()> {
    //let mut db = Connection::open("10m.db")?;
    let mut db = Connection::open_in_memory()?;

    let create_table_sql = r"
                      CREATE TABLE item (
                          id INTEGER NOT NULL,
                          original VARCHAR NOT NULL,
                          descendants INTEGER,
                          username VARCHAR,
                          score INTEGER,
                          title VARCHAR,
                          url VARCHAR,
                          body VARCHAR,
                          ts TIMESTAMP
                  );";
    db.execute_batch(create_table_sql)?;

    // let mut tx = db.transaction()?;
    // tx.set_drop_behavior(DropBehavior::Commit);
    let mut app = db.appender("item")?;

    let row_count = 10;
    for i in 0..row_count {
        app.append_row(params![
            i,
            "",
            1,
            "username",
            1,
            "title",
            "url",
            "body",
            "2020-01-01T00:00:00Z"
        ])?;
    }

    let val = db.query_row("SELECT count(1) FROM item", [], |row| {
        <(u32,)>::try_from(row)
    })?;

    assert_eq!(val, (row_count,));

    Ok(())
}
