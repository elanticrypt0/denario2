use diesel::{prelude::*};
use chrono::{NaiveDateTime,NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Insertable,Queryable, Selectable, Serialize, AsChangeset, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::credits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Credit{
    pub id:i32,
    pub name:String,
    pub comment:Option<String>,
    pub amount:f64,
    pub payments:i32,
    pub started_at:NaiveDate,
    pub finish_at:NaiveDate,
    pub category_id:i32,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub is_deleted:bool,
}