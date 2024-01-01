use rusqlite::OptionalExtension;

use super::*;

pub struct Sessions<'a> {
    conn: &'a Connection,
}

impl<'a> Sessions<'a> {
    pub(super) fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    fn check_expiry(&self, cookie: &str) {
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT expiry FROM sessions WHERE cookie = ?1")
            .expect("Should be a valid sql statement");

        let time = get_timestamp();

        let expiry = match stmnt.query_row(params![cookie], |row| row.get::<usize, u64>(0)) {
            Ok(mapped) => mapped,
            Err(_) => return,
        };

        if time > expiry as u128 {
            let mut stmnt = self
                .conn
                .prepare_cached("UPDATE sessions SET invalid = 1 WHERE cookie = ?1")
                .expect("Should be a valid sql statement");

            match stmnt.execute(params![cookie]) {
                _ => return,
            };
        }
    }

    pub fn end_session(&self, cookie: &str) -> bool {
        let mut stmnt = self
            .conn
            .prepare_cached("UPDATE sessions SET invalid = 1 WHERE cookie = ?1")
            .expect("Should be a valid sql statement");
        stmnt.execute(params![cookie]).is_ok()
    }

    pub fn validate_session(&self, cookie: &str) -> bool {
        self.check_expiry(cookie);
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT invalid FROM sessions WHERE cookie = ?1")
            .expect("Should be a valid sql statement");

        match stmnt
            .query_row(params![cookie], |row| row.get::<usize, bool>(0))
            .optional()
        {
            Ok(opt) => match opt {
                None => false,
                Some(invalid) => !invalid,
            },
            Err(_) => return false,
        }
    }

    pub fn create_new_session(&self, ip: SocketAddr) -> Result<String> {
        // Convert user IP to string
        let ip_str = ip.to_string();

        // Get the current timestamp
        let time = get_timestamp();

        // sha512(IP + Timestamp)
        let combine = format!("{ip_str}{time}");

        let mut hasher = <Sha512 as Digest>::new();
        Digest::update(&mut hasher, combine.as_bytes());
        let digest = Digest::finalize(hasher);

        // random 32 bytes of data
        let mut rand = [0u8; 32];
        thread_rng().fill(&mut rand);

        // Digest + Random
        let mut result = digest.to_vec();
        result.append(&mut rand.to_vec());

        // Encode as Base64
        let encoded = general_purpose::STANDARD.encode(result);

        // Enable Session in SQLite DB
        let mut stmnt = self
            .conn
            .prepare_cached("INSERT INTO sessions (cookie) VALUES (?1)")
            .expect("Should be a valid sql statement");

        match stmnt.execute(params![encoded]) {
            Ok(_) => (),
            Err(_) => bail!(SQLError),
        };

        Ok(encoded)
    }

    pub fn user_info_from_cookie(&self, cookie: &str) -> Option<UserInfo> {
        self.check_expiry(&cookie);

        let mut stmnt = self
            .conn
            .prepare_cached(
                "SELECT u.handle, u.display FROM sessions s INNER JOIN users u ON u.id = s.user WHERE s.cookie = ?1 AND s.invalid = 0",
            )
            .expect("Should be a valid sql statement");
        let handle_result = stmnt.query_row(params![cookie], |row| {
            Ok((row.get::<usize, String>(0), row.get::<usize, String>(1)))
        });

        let handle_result = match handle_result {
            Ok(o) => o,
            Err(_) => return Some(UserInfo::new_guest()),
        };

        match handle_result {
            (Err(_), _) => Some(UserInfo::new_guest()),
            (Ok(handle), display) => Some(UserInfo::new_user(handle, display.unwrap_or("ERROR".to_string()))),
        }
    }

    /// ### Assigns a session to a user
    ///
    /// Returns `true` if successful
    ///
    /// Returns `false` if that session already has an assigned user
    pub fn assign_session_user(&self, cookie: &str, handle: String) -> Result<bool> {
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT user FROM sessions WHERE cookie = ?1 LIMIT 1")
            .expect("Should be a valid sql statement");

        let user = stmnt.query_row(params![cookie], |row| row.get::<usize, Option<u64>>(0))?;

        if let Some(_) = user {
            return Ok(false);
        }

        let mut stmnt = self
            .conn
            .prepare_cached(
                "UPDATE sessions SET user = (SELECT id FROM users WHERE handle = ?1 LIMIT 1) WHERE cookie = ?2",
            )
            .expect("Should be a valid sql statement");

        stmnt.execute(params![handle, cookie])?;

        Ok(true)
    }
}
