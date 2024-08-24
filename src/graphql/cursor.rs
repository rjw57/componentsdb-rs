use base64::prelude::{Engine, BASE64_URL_SAFE_NO_PAD};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    pg::{Pg, PgValue},
};
use juniper::ID;
use uuid::Uuid;

#[derive(PartialEq, FromSqlRow)]
pub struct Cursor(Uuid);

impl TryFrom<ID> for Cursor {
    type Error = &'static str;

    fn try_from(value: ID) -> Result<Self, Self::Error> {
        let bytes = BASE64_URL_SAFE_NO_PAD
            .decode(value.to_string())
            .or(Err("Cursor could not be decoded into bytes"))?;
        let uuid = Uuid::from_slice(bytes.as_slice()).or(Err("Cursor is not a UUID"))?;
        Ok(Cursor(uuid))
    }
}

impl Into<ID> for Cursor {
    fn into(self) -> ID {
        BASE64_URL_SAFE_NO_PAD.encode(self.0.as_bytes()).into()
    }
}

impl Into<Uuid> for Cursor {
    fn into(self) -> Uuid {
        self.0
    }
}

impl FromSql<diesel::sql_types::Uuid, Pg> for Cursor {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let uuid = Uuid::from_sql(bytes)?;
        Ok(Cursor(uuid))
    }
}
