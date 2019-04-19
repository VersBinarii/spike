use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "numbers"]
#[primary_key(number_id)]
pub struct NewNumber {
    number: String,
    number_type_id: i32,
    number_status_id: i32,
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

#[derive(Debug, Queryable, Serialize)]
pub struct Number {
    number_id: i32,
    number: String,
    number_type_id: i32,
    number_status_id: i32,
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

#[derive(Queryable, Serialize, Deserialize)]
pub struct NumberBlock {
    numberblock_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Porting {
    porting_id: i32,
    number_id: i32,
    porting_from: String,
    porting_to: String,
    porting_status_id: i32,
    porting_start: NaiveDateTime,
    porting_event_date: Option<NaiveDateTime>,
    porting_completion: Option<NaiveDateTime>,
    comments: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PortingStatus {
    porting_status_id: i32,
    porting_status: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Subscriber {
    subscriber_id: i32,
    address_id: Option<i32>,
    rsp_id: i32,
    name: String,
    accountid: Option<String>,
    is_business: bool,
    premise_id: Option<String>,
    eircode_id: Option<String>,
    ard_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Rsp {
    rsp_id: i32,
    address_id: Option<i32>,
    name: String,
    accountid: Option<String>,
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
    latitude: Option<f32>,
    longitude: Option<f32>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct RoutingPrefix {
    prefix_id: i32,
    prefix: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct NumberType {
    number_type_id: i32,
    number_type: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct NumberStatus {
    number_status_id: i32,
    number_status: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Mna {
    mna_id: i32,
    area_code: String,
    digits: i32,
    mna: String,
    towns: String,
    area: String,
}
