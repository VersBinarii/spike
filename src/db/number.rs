use actix::prelude::*;
use diesel::prelude::*;

use crate::db::pagination::*;
use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

pub struct FetchNumber(pub i32);

impl Message for FetchNumber {
    type Result = Result<models::number::Number, SpikeError>;
}

impl Handler<FetchNumber> for DbExecutor {
    type Result = Result<models::number::Number, SpikeError>;

    fn handle(
        &mut self,
        msg: FetchNumber,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::numbers::dsl::*;

        match numbers
            .filter(number_id.eq(msg.0))
            .limit(1)
            .load::<models::number::Number>(&self.pool.get()?)
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

pub struct FetchNumbers {
    pub page: i64,
    pub per_page: i64,
}

impl Message for FetchNumbers {
    type Result = Result<(Vec<models::number::Number>, i64), SpikeError>;
}

impl Handler<FetchNumbers> for DbExecutor {
    type Result = Result<(Vec<models::number::Number>, i64), SpikeError>;
    fn handle(
        &mut self,
        msg: FetchNumbers,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::numbers::dsl::*;

        match numbers
            .select(numbers::all_columns())
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<models::number::Number>(&*self.pool.get()?)
        {
            Ok(nums) => Ok(nums),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct InsertNumber(pub models::number::NewNumber);

impl Message for InsertNumber {
    type Result = Result<models::number::Number, SpikeError>;
}

impl Handler<InsertNumber> for DbExecutor {
    type Result = Result<models::number::Number, SpikeError>;
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
    pub number: models::number::NewNumber,
    pub number_id: i32,
}

impl Message for UpdateNumber {
    type Result = Result<models::number::Number, SpikeError>;
}

impl Handler<UpdateNumber> for DbExecutor {
    type Result = Result<models::number::Number, SpikeError>;
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
            Ok(updated_number) => Ok(updated_number),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
