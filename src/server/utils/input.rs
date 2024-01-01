use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Invalid { context: &'static str },

    MinLength { expect: u8 },

    MaxLength { expect: u8 },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Invalid { context } => format!("InvalidError: {context}"),
                Self::MinLength { expect } => format!("LengthError: Must have at least {expect} characters present"),
                Self::MaxLength { expect } => format!("LengthError: Must have at most {expect} characters present"),
            }
        )
    }
}

impl std::error::Error for Error {}

static VALID_HANDLE_PATTERN: [char; 37] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_',
];

/// ### Validates the passed handle according to the following requirements
///
/// 1. Handles must only contain the following pattern: /^[a-z0-9_]*$/
/// 2. Handles must be at least 2 characters long
/// 3. Handles must be at most 32 characters long
pub fn validate_handle(handle: &str) -> Result<(), Error> {
    if !handle.chars().all(|c| VALID_HANDLE_PATTERN.contains(&c)) {
        return Err(Error::Invalid {
            context: "Must only contain a-z, 0-9, and underscores",
        });
    }

    check_length(handle, 2, 32)?;

    Ok(())
}

static INVALID_DISPLAY_PATTERN: [char; 8] = ['&', '<', '>', '+', ',', '!', ';', '.'];
/// ### Validates the passed display name according to the following requirements
///
/// 1. Display names must not contain the following characters:
///     1. '&'
///     2. '<'
///     3. '\>'
///     4. '+'
///     5. ','
///     6. '!'
///     7. ';'
///     8. '.'
/// 2. Display names must be at least 1 character long
/// 3. Display names must be at most 32 characters long
pub fn validate_display(display: &str) -> Result<(), Error> {
    if display.contains(INVALID_DISPLAY_PATTERN) {
        return Err(Error::Invalid {
            context: "Must not contain anything from the following set: &<>+,.!;",
        });
    }

    check_length(&display, 1, 32)?;

    Ok(())
}

/// ### Validates the passed password according to the following requirements
///
/// 1. Passwords must be at least 8 characters long
/// 2. Passwords must not be more than 128 characters long
pub fn validate_password(password: &str) -> Result<(), Error> {
    check_length(&password, 8, 128)?;

    Ok(())
}

fn check_length(string: &str, min: usize, max: usize) -> Result<(), Error> {
    let length = string.chars().count();

    if length < min {
        return Err(Error::MinLength { expect: min as u8 });
    }

    if length > max {
        return Err(Error::MaxLength { expect: max as u8 });
    }

    Ok(())
}
