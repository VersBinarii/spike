use crate::schema::*;
use chrono::NaiveDateTime;

pub mod number;
pub mod porting;

#[derive(Queryable, Serialize, Deserialize)]
pub struct NumberBlock {
    numberblock_id: i32,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "subscribers"]
#[primary_key(subscriber_id)]
pub struct NewSubscriber {
    address_id: Option<i32>,
    rsp_id: Option<i32>,
    name: String,
    accountid: String,
    is_business: bool,
    premise_id: Option<String>,
    eircode_id: Option<String>,
    ard_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Subscriber {
    subscriber_id: i32,
    address_id: Option<i32>,
    rsp_id: Option<i32>,
    name: String,
    accountid: String,
    is_business: bool,
    premise_id: Option<String>,
    eircode_id: Option<String>,
    ard_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "rsp"]
#[primary_key(rsp_id)]
pub struct NewRsp {
    address_id: Option<i32>,
    name: String,
    account: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Rsp {
    rsp_id: i32,
    address_id: Option<i32>,
    name: String,
    account: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "address"]
#[primary_key(address_id)]
pub struct NewAddress {
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    county: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    business_name: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Address {
    address_id: i32,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    county: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    business_name: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct RoutingPrefix {
    prefix_id: i32,
    prefix: String,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "mna"]
#[primary_key(mna_id)]
pub struct NewMna {
    area_code: String,
    digits: i32,
    description: String,
    towns: String,
    area: String,
}

#[derive(Debug, Queryable, Serialize, Associations, Deserialize)]
pub struct Mna {
    mna_id: i32,
    area_code: String,
    digits: i32,
    description: String,
    towns: String,
    area: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Queryable, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Queryable, Deserialize)]
pub struct LogoutUser {
    pub username: String,
}

#[derive(Debug, Queryable, Insertable, Deserialize, Serialize)]
pub struct Token {
    token_id: String,
    username: String,
    expiry: NaiveDateTime,
}
