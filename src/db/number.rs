use actix::prelude::*;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

pub struct FetchNumber(pub i32);

impl Message for FetchNumber {
    type Result = Result<models::Number, SpikeError>;
}

impl Handler<FetchNumber> for DbExecutor {
    type Result = Result<models::Number, SpikeError>;

    fn handle(
        &mut self,
        msg: FetchNumber,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::numbers::dsl::*;

        match numbers
            .filter(number_id.eq(msg.0))
            .limit(1)
            .load::<models::Number>(&self.pool.get()?)
        {
            Ok(mut nums) => {
                if let Some(num) = nums.pop() {
                    Ok(num)
                } else {
                    Err(SpikeError::InvalidId)
                }
            }
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct FetchNumbers;

impl Message for FetchNumbers {
    type Result = Result<Vec<models::Number>, SpikeError>;
}

impl Handler<FetchNumbers> for DbExecutor {
    type Result = Result<Vec<models::Number>, SpikeError>;
    fn handle(
        &mut self,
        _: FetchNumbers,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::numbers::dsl::*;

        match numbers.load::<models::Number>(&self.pool.get()?) {
            Ok(nums) => Ok(nums),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct InsertNumber(pub models::NewNumber);

impl Message for InsertNumber {
    type Result = Result<models::Number, SpikeError>;
}

impl Handler<InsertNumber> for DbExecutor {
    type Result = Result<models::Number, SpikeError>;
    fn handle(
        &mut self,
        msg: InsertNumber,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::numbers::dsl::*;

        match diesel::insert_into(numbers)
            .values(msg.0)
            .get_result(&self.pool.get()?)
        {
            Ok(inserted_number) => Ok(inserted_number),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct UpdateNumber {
    pub number: models::NewNumber,
    pub number_id: i32,
}

impl Message for UpdateNumber {
    type Result = Result<models::Number, SpikeError>;
}

impl Handler<UpdateNumber> for DbExecutor {
    type Result = Result<models::Number, SpikeError>;
    fn handle(
        &mut self,
        msg: UpdateNumber,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::numbers::dsl::*;

        match diesel::update(numbers)
            .filter(number_id.eq(&msg.number_id))
            .set(&msg.number)
            .get_result(&self.pool.get()?)
        {
            Ok(inserted_number) => Ok(inserted_number),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
