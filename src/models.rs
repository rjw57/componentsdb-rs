use chrono::{DateTime, Utc};
use diesel::prelude::*;
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

#[derive(Insertable)]
#[diesel(table_name = crate::schema::cabinets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCabinet<'a> {
    pub name: &'a str,
}

pub fn create_cabinet(conn: &mut PgConnection, name: &str) -> Cabinet {
    use crate::schema::cabinets;

    let new_cabinet = NewCabinet { name };

    diesel::insert_into(cabinets::table)
        .values(&new_cabinet)
        .returning(Cabinet::as_returning())
        .get_result(conn)
        .expect("Error saving new cabinet")
}

#[cfg(test)]
mod test {
    use fake::{Fake, Faker};

    use super::*;
    use crate::test::within_test_transaction;

    fn fake_cabinet(conn: &mut PgConnection) -> Cabinet {
        create_cabinet(conn, &Faker.fake::<String>())
    }

    #[test]
    fn can_create() {
        within_test_transaction(|conn| {
            assert_ne!(fake_cabinet(conn).id, 0);
        })
    }
}
