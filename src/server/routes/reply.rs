use serde::Serialize;

#[derive(Serialize)]
pub enum Status {
    Success { context: Option<String> },
    Failure { context: Option<String> },
    Partial { context: Option<String> },
}

impl From<anyhow::Error> for Status {
    fn from(value: anyhow::Error) -> Self {
        Self::Failure {
            context: Some(value.to_string()),
        }
    }
}

#[derive(Serialize)]
pub enum Message {
    SignUp { status: Status },
    Login { status: Status },
}
