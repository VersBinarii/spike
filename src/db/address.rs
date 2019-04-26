use actix::prelude::*;
use diesel::prelude::*;

use crate::db::pagination::*;
use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

pub struct FetchAddress(pub i32);

impl Message for FetchAddress {
    type Result = Result<models::Address, SpikeError>;
}

impl Handler<FetchAddress> for DbExecutor {
    type Result = Result<models::Address, SpikeError>;

    fn handle(
        &mut self,
        msg: FetchAddress,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::address::dsl::*;

        match address
            .filter(address_id.eq(msg.0))
            .limit(1)
            .load::<models::Address>(&self.pool.get()?)
        {
            Ok(mut ret_address) => {
                if let Some(the_address) = ret_address.pop() {
                    Ok(the_address)
                } else {
                    Err(SpikeError::InvalidId)
                }
            }
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct FetchAddresses {
    pub page: i64,
    pub per_page: i64,
}

impl Message for FetchAddresses {
    type Result = Result<(Vec<models::Address>, i64), SpikeError>;
}

impl Handler<FetchAddresses> for DbExecutor {
    type Result = Result<(Vec<models::Address>, i64), SpikeError>;
    fn handle(
        &mut self,
        msg: FetchAddresses,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::address::dsl::*;

        match address
            .select(address::all_columns())
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<models::Address>(&*self.pool.get()?)
        {
            Ok(addresses) => Ok(addresses),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct InsertAddress(pub models::NewAddress);

impl Message for InsertAddress {
    type Result = Result<models::Address, SpikeError>;
}

impl Handler<InsertAddress> for DbExecutor {
    type Result = Result<models::Address, SpikeError>;
    fn handle(
        &mut self,
        msg: InsertAddress,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::address::dsl::*;

        match diesel::insert_into(address)
            .values(msg.0)
            .get_result(&self.pool.get()?)
        {
            Ok(inserted_address) => Ok(inserted_address),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct UpdateAddress {
    pub address: models::NewAddress,
    pub address_id: i32,
}

impl Message for UpdateAddress {
    type Result = Result<models::Address, SpikeError>;
}

impl Handler<UpdateAddress> for DbExecutor {
    type Result = Result<models::Address, SpikeError>;
    fn handle(
        &mut self,
        msg: UpdateAddress,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::address::dsl::*;

        match diesel::update(address)
            .filter(address_id.eq(&msg.address_id))
            .set(&msg.address)
            .get_result(&self.pool.get()?)
        {
            Ok(updated_address) => Ok(updated_address),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
