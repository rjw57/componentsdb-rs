use chrono::{DateTime, Utc};
use diesel::{connection::LoadConnection, pg::Pg, prelude::*};
use fake::{Dummy, Fake, Faker};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cabinets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cabinet {
    pub id: i64,
    pub name: String,
    pub uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Dummy)]
#[diesel(table_name = crate::schema::cabinets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCabinet {
    pub name: String,
}

impl Cabinet {
    pub fn fake<Conn>(conn: &mut Conn) -> QueryResult<Cabinet>
    where
        Conn: Connection<Backend = Pg> + LoadConnection,
    {
        diesel::insert_into(crate::schema::cabinets::table)
            .values(fake_cabinet())
            .returning(Cabinet::as_returning())
            .get_result(conn)
    }
}

pub fn fake_cabinet() -> NewCabinet {
    Faker.fake()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::*;

    #[test]
    fn can_create_cabinet() {
        with_db_conn(|conn| {
            assert_ne!(Cabinet::fake(conn).unwrap().id, 0);
        })
    }
}
