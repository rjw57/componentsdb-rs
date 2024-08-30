use diesel::prelude::*;
use juniper::*;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::db::*;
pub use cursor::*;

mod cursor;

#[derive(GraphQLObject, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::cabinets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[graphql(description = "A cabinet which holds multiple drawers of components")]
pub struct Cabinet {
    #[diesel(column_name = uuid, deserialize_as = Cursor)]
    id: ID,
    name: String,
}

pub struct Context {
    pub db_conn_mutex: Arc<Mutex<DbPooledConnection>>,
}

impl Context {
    pub fn with_db_conn<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut DbPooledConnection) -> T,
    {
        let mut conn = self.db_conn_mutex.lock().unwrap();
        f(&mut *conn)
    }
}

impl juniper::Context for Context {}

struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn cabinet(id: ID, context: &Context) -> FieldResult<Cabinet> {
        use super::schema::cabinets::dsl::{cabinets, uuid};

        let cursor: Cursor = id.try_into()?;
        let cursor_uuid: Uuid = cursor.into();
        let cabinet = context.with_db_conn(|conn| {
            cabinets
                .filter(uuid.eq(cursor_uuid))
                .select(Cabinet::as_select())
                .first(conn)
                .optional()
        })?;

        match cabinet {
            Some(cabinet) => Ok(cabinet),
            None => Err("No cabinet found".into()),
        }
    }
}

// type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::models;
    use crate::testing::*;

    fn with_context<F>(f: F)
    where
        F: FnOnce(&Context),
    {
        let mut conn = get_db_connection();
        conn.begin_test_transaction().ok();
        migrate(&mut conn).ok();
        let db_conn_mutex = Arc::new(Mutex::new(conn));
        let context = Context { db_conn_mutex };
        f(&context)
    }

    #[test]
    fn can_create_context() {
        with_context(|_| {})
    }

    #[test]
    fn can_query_nonexistant_cabinet() {
        with_context(|context| {
            let cursor: Cursor = uuid::Uuid::new_v4().into();
            let cabinet = Query::cabinet(cursor.into(), &context);
            assert!(cabinet.is_err());
        });
    }

    #[test]
    fn can_query_cabinet() {
        with_context(|context| {
            let db_cabinet = context.with_db_conn(|conn| models::Cabinet::fake(conn).unwrap());
            let cursor: Cursor = db_cabinet.uuid.into();
            let gql_cabinet = Query::cabinet(cursor.into(), &context).unwrap();
            assert_eq!(gql_cabinet.name, db_cabinet.name);
        })
    }
}
