use super::*;

pub struct Auth<'a> {
    conn: &'a Connection,
    argon2: Argon2<'a>,
}

impl<'a> Auth<'a> {
    pub(super) fn new(conn: &'a Connection, argon2: Argon2<'a>) -> Self {
        Self { conn, argon2 }
    }

    /// ### Attempt to get a user's associated PHC string
    ///
    /// Returns an `Option<String>`
    ///
    /// Will be `None` if no user is found
    ///
    /// Will be `Some(String)` if a user is found
    fn get_phc(&self, handle: String) -> Option<String> {
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT phc FROM users WHERE handle = ?1")
            .expect("Should be a valid sql statement");

        match stmnt.query_row(params![handle], |row| Ok(row.get_unwrap("phc"))) {
            Err(_) => None,
            Ok(r) => Some(r),
        }
    }

    /// ### Verifies if a particular user exists or not
    pub fn user_exists(&self, handle: String) -> Result<bool> {
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT EXISTS(SELECT 1 FROM users WHERE handle = ?1 LIMIT 1)")
            .expect("Should be valid sql");

        let result: bool = stmnt.query_row(params![handle], |row| row.get(0))?;

        Ok(result)
    }

    /// ### Creates a user from the provided handle, display name, and password
    ///
    /// All values MUST be validated BEFORE calling this function
    ///
    /// Returns an `anyhow::Result<bool>`
    ///
    /// If `Ok(bool)`, the bool indicates if the username is taken,
    /// true for user created, false for username taken
    pub fn create_user(&self, handle: String, display: String, password: String) -> Result<bool> {
        if let None = self.get_phc(handle.clone()) {
            let password = password.as_bytes();
            let salt = SaltString::generate(&mut OsRng);

            let phc = match self.argon2.hash_password(password, &salt) {
                Err(e) => bail!(ArgonError::from(e)),
                Ok(pwdh) => pwdh.to_string(),
            };

            let mut stmnt = self
                .conn
                .prepare_cached("INSERT INTO users (handle, display, phc) VALUES (?1, ?2, ?3)")
                .expect("Should be a valid sql statement");

            match stmnt.execute(params![handle, display, phc]) {
                Err(_) => bail!(SQLError),
                Ok(_) => (),
            };

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// ### Validate a user login the user's handle and password
    ///
    /// Returns an `anyhow::Result<bool>`
    ///
    /// If `Ok(bool)`, the bool indicates if the login is valid,
    /// true for valid, false for invalid
    pub fn validate_user(&self, handle: String, password: String) -> Result<bool> {
        match self.get_phc(handle) {
            None => Ok(false),
            Some(phc) => {
                let password = password.as_bytes();
                let hash = match PasswordHash::new(&phc) {
                    Err(e) => bail!(ArgonError::from(e)),
                    Ok(pwdh) => pwdh,
                };
                Ok(self.argon2.verify_password(&password, &hash).is_ok())
            }
        }
    }

    /// ### Update a user's password, requiring the previous password
    pub fn update_password(&self) {}
}

#[derive(Debug)]
pub enum ArgonError {
    HashError(password_hash::Error),
    EncodingError(password_hash::Error),
    UnknownError(password_hash::Error),
}

impl From<password_hash::Error> for ArgonError {
    fn from(value: password_hash::Error) -> Self {
        match value {
            password_hash::Error::SaltInvalid(_) => Self::HashError(value),
            password_hash::Error::Algorithm => Self::HashError(value),
            password_hash::Error::B64Encoding(_) => Self::EncodingError(value),
            password_hash::Error::Crypto => Self::HashError(value),
            password_hash::Error::Version => Self::HashError(value),
            _ => Self::UnknownError(value),
        }
    }
}

impl Display for ArgonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::HashError(err) => format!("HashError: {}", err),
                Self::EncodingError(err) => format!("EncodingError: {}", err),
                Self::UnknownError(err) => format!("UnknownError: {}", err),
            }
        )
    }
}

impl Error for ArgonError {}
