use diesel::prelude::*;
use juniper::*;
use std::sync::Arc;
use uuid::Uuid;

use super::db;
pub use cursor::*;

mod cursor;

#[derive(GraphQLObject, Queryable, Selectable)]
#[diesel(table_name = crate::schema::cabinets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[graphql(description = "A cabinet which holds multiple drawers of components")]
pub struct Cabinet {
    #[diesel(column_name = uuid, deserialize_as = Cursor)]
    id: ID,
    name: String,
}

#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<db::DbPool>,
}

impl juniper::Context for Context {}

struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn cabinet(id: ID, context: &Context) -> FieldResult<Cabinet> {
        use super::schema::cabinets::dsl::{cabinets, uuid};

        let conn = &mut context.db_pool.get().unwrap();
        let cursor: Cursor = id.try_into()?;
        let cursor_uuid: Uuid = cursor.into();
        let cabinet = cabinets
            .filter(uuid.eq(cursor_uuid))
            .select(Cabinet::as_select())
            .first(conn)
            .optional()?;

        match cabinet {
            Some(cabinet) => Ok(cabinet),
            None => Err("No cabinet found".into()),
        }
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::*;

    #[test]
    fn can_create_context() {
        Context {
            db_pool: Arc::new(get_db_pool()),
        };
    }
}
