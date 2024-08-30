use diesel::prelude::*;
use juniper::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::cabinets)]
pub struct Cabinet {
    uuid: Uuid,
    name: String,
}

#[graphql_object]
#[graphql(description = "A cabinet which holds multiple drawers of components")]
impl Cabinet {
    fn id(&self) -> ID {
        ID::new(self.uuid)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
