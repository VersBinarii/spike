use crate::models::Mna;
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
    Debug, Deserialize, Serialize, Eq, PartialEq, AsExpression, FromSqlRow,
)]
#[sql_type = "Integer"]
pub enum NumType {
    Geo = 1,
    NonGeo = 2,
    VoIP = 3,
}

impl From<i32> for NumType {
    fn from(input: i32) -> Self {
        match input {
            1 => NumType::Geo,
            2 => NumType::NonGeo,
            3 => NumType::VoIP,
            _n => panic!("Unknown type"),
        }
    }
}

impl<DB: Backend> ToSql<Integer, DB> for NumType
where
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: Write,
    {
        let v = match *self {
            NumType::Geo => 1,
            NumType::NonGeo => 2,
            NumType::VoIP => 3,
        };
        v.to_sql(out)
    }
}

impl FromSql<Integer, Pg> for NumType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <i32 as FromSql<Integer, Pg>>::from_sql(bytes)? {
            1 => Ok(NumType::Geo),
            2 => Ok(NumType::NonGeo),
            3 => Ok(NumType::VoIP),
            n => Err(format!("Unknown number type: {}", n).into()),
        }
    }
}

#[derive(
    Debug, Deserialize, Serialize, Eq, PartialEq, AsExpression, FromSqlRow,
)]
#[sql_type = "Integer"]
pub enum NumStatus {
    Available = 1,
    Reserved = 2,
    Assigned = 3,
    Quarantined = 4,
    PortPending = 5,
    PortedIn = 6,
    PortedOut = 7,
}

impl From<i32> for NumStatus {
    fn from(input: i32) -> Self {
        match input {
            1 => NumStatus::Available,
            2 => NumStatus::Reserved,
            3 => NumStatus::Assigned,
            4 => NumStatus::Quarantined,
            5 => NumStatus::PortPending,
            6 => NumStatus::PortedIn,
            7 => NumStatus::PortedOut,
            _n => panic!("Unknown type"),
        }
    }
}

impl<DB: Backend> ToSql<Integer, DB> for NumStatus
where
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: Write,
    {
        let v = match *self {
            NumStatus::Available => 1,
            NumStatus::Reserved => 2,
            NumStatus::Assigned => 3,
            NumStatus::Quarantined => 4,
            NumStatus::PortPending => 5,
            NumStatus::PortedIn => 6,
            NumStatus::PortedOut => 7,
        };
        v.to_sql(out)
    }
}

impl FromSql<Integer, Pg> for NumStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <i32 as FromSql<Integer, Pg>>::from_sql(bytes)? {
            1 => Ok(NumStatus::Available),
            2 => Ok(NumStatus::Reserved),
            3 => Ok(NumStatus::Assigned),
            4 => Ok(NumStatus::Quarantined),
            5 => Ok(NumStatus::PortPending),
            6 => Ok(NumStatus::PortedIn),
            7 => Ok(NumStatus::PortedOut),
            n => Err(format!("Unknown number status code: {}", n).into()),
        }
    }
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "numbers"]
#[primary_key(number_id)]
pub struct NewNumber {
    number: String,
    number_type: NumType,
    number_status: NumStatus,
    prefix_id: Option<i32>,
    status_change: Option<NaiveDateTime>,
    block_holder: Option<String>,
    comments: Option<String>,
    mna_id: i32,
    subscriber_id: Option<i32>,
    numberblock_id: Option<i32>,
    wlr: bool,
    is_main_number: bool,
    ecas: bool,
    ndd: bool,
}

#[derive(Debug, Queryable, Associations, Serialize)]
#[belongs_to(Mna)]
pub struct Number {
    number_id: i32,
    number: String,
    number_type: NumType,
    number_status: NumStatus,
    prefix_id: Option<i32>,
    status_change: Option<NaiveDateTime>,
    block_holder: Option<String>,
    comments: Option<String>,
    mna_id: i32,
    subscriber_id: Option<i32>,
    numberblock_id: Option<i32>,
    wlr: bool,
    is_main_number: bool,
    ecas: bool,
    ndd: bool,
}
