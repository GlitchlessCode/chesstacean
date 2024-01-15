use super::*;

pub struct Games<'a> {
    conn: &'a Connection,
}

impl<'a> Games<'a> {
    pub(super) fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn game_exists(&self, id: String) -> Result<bool> {
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT EXISTS(SELECT 1 FROM games WHERE name = ?1 LIMIT 1)")
            .expect("Should be a valid sql statement");

        let result: bool = stmnt.query_row(params![id], |row| row.get(0))?;

        Ok(result)
    }
}
