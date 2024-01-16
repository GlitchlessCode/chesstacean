use serde::Serialize;

#[derive(Serialize)]
pub enum Message<T>
where
    T: Serialize,
{
    Error { message: String, affects: T },
}
