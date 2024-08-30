use diesel::prelude::*;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode, ID};
use uuid::Uuid;

pub use context::*;
pub use cursor::*;
pub use schema::*;

mod context;
mod cursor;
mod schema;
#[cfg(test)]
mod testing;

pub struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn cabinet(id: ID, context: &Context) -> FieldResult<Cabinet> {
        use super::schema::cabinets::dsl::{cabinets, uuid};

        let uuid_: Uuid = (*id).try_into()?;
        context
            .with_db_conn(|conn| {
                cabinets
                    .filter(uuid.eq(uuid_))
                    .select(Cabinet::as_select())
                    .first(conn)
            })
            .or(Err("No such cabinet".into()))
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn new_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

#[cfg(test)]
mod test {
    use juniper::{graphql_value, graphql_vars, Value};

    use crate::models;

    use super::testing::*;
    use super::*;

    #[test]
    fn can_create_context() {
        with_context(|_| {});
    }

    fn query_cabinet(id: &str, context: &Context) -> Value {
        let (res, errors) = execute_sync(
            "query($id: ID!) { cabinet(id: $id) { name } }",
            &graphql_vars! {
                "id": (id),
            },
            context,
        )
        .unwrap();
        assert_eq!(errors.len(), 0);
        res
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
            let id: String = db_cabinet.uuid.into();
            let res = query_cabinet(&id, context);
            assert_eq!(
                res,
                graphql_value!({
                    "cabinet": {
                        "name": db_cabinet.name,
                    }
                })
            )
        });
    }
}
