use std::sync::{Arc, Mutex};

use juniper::{DefaultScalarValue, ExecutionError, GraphQLError, Value, Variables};

use super::*;
use crate::db::migrate;
use crate::testing::*;

pub fn with_context<F>(f: F)
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

pub fn execute_sync(
    document_source: &str,
    variables: &Variables,
    context: &Context,
) -> Result<(Value, Vec<ExecutionError<DefaultScalarValue>>), GraphQLError> {
    juniper::execute_sync(document_source, None, &new_schema(), variables, context)
}
