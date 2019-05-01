use crate::models::number::Number;
use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::Integer,
};
use std::io::Write;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, AsExpression, FromSqlRow,
)]
#[sql_type = "Integer"]
pub enum PortingStatus {
    PortingStart = 1,
    PortingComplete = 2,
    PortingRejected = 3,
}

impl From<i32> for PortingStatus {
    fn from(input: i32) -> Self {
        match input {
            1 => PortingStatus::PortingStart,
            2 => PortingStatus::PortingComplete,
            3 => PortingStatus::PortingRejected,
            _ => panic!("Unknown type"),
        }
    }
}

impl<DB: Backend> ToSql<Integer, DB> for PortingStatus
where
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: Write,
    {
        let v = match *self {
            PortingStatus::PortingStart => 1,
            PortingStatus::PortingComplete => 2,
            PortingStatus::PortingRejected => 3,
        };
        v.to_sql(out)
    }
}

impl FromSql<Integer, Pg> for PortingStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <i32 as FromSql<Integer, Pg>>::from_sql(bytes)? {
            1 => Ok(PortingStatus::PortingStart),
            2 => Ok(PortingStatus::PortingComplete),
            3 => Ok(PortingStatus::PortingRejected),
            n => Err(format!("Unknown porting status code: {}", n).into()),
        }
    }
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "portings"]
#[primary_key(porting_id)]
pub struct NewPorting {
    number_id: Option<i32>,
    numberblock_id: Option<i32>,
    porting_from: String,
    porting_to: String,
    status: PortingStatus,
    porting_start: NaiveDateTime,
    porting_event_date: Option<NaiveDateTime>,
    porting_completion: Option<NaiveDateTime>,
    comments: Option<String>,
}

#[derive(Debug, Queryable, Associations, Serialize)]
#[belongs_to(Number)]
pub struct Porting {
    porting_id: i32,
    number_id: Option<i32>,
    numberblock_id: Option<i32>,
    porting_from: String,
    porting_to: String,
    status: PortingStatus,
    porting_start: NaiveDateTime,
    porting_event_date: Option<NaiveDateTime>,
    porting_completion: Option<NaiveDateTime>,
    comments: Option<String>,
}
