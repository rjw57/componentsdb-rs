use base64::prelude::{Engine, BASE64_URL_SAFE_NO_PAD};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    pg::{Pg, PgValue},
};
use juniper::ID;
use uuid::Uuid;

#[derive(PartialEq, FromSqlRow)]
pub struct Cursor(Uuid);

impl Cursor {
    pub fn as_uuid(self) -> Uuid {
        self.into()
    }

    pub fn as_id(self) -> ID {
        self.into()
    }
}

impl From<Uuid> for Cursor {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<ID> for Cursor {
    type Error = anyhow::Error;

    fn try_from(value: ID) -> Result<Self, Self::Error> {
        let bytes = BASE64_URL_SAFE_NO_PAD.decode(value.to_string())?;
        let uuid = Uuid::from_slice(bytes.as_slice())?;
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
        self.as_uuid()
    }
}

impl FromSql<diesel::sql_types::Uuid, Pg> for Cursor {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let uuid = Uuid::from_sql(bytes)?;
        Ok(Cursor(uuid))
    }
}
