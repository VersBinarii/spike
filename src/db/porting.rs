use actix::prelude::*;
use diesel::prelude::*;

use crate::db::pagination::*;
use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

pub struct FetchPorting(pub i32);

impl Message for FetchPorting {
    type Result = Result<models::porting::Porting, SpikeError>;
}

impl Handler<FetchPorting> for DbExecutor {
    type Result = Result<models::porting::Porting, SpikeError>;

    fn handle(
        &mut self,
        msg: FetchPorting,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::portings::dsl::*;

        match portings
            .filter(porting_id.eq(msg.0))
            .limit(1)
            .load::<models::porting::Porting>(&self.pool.get()?)
        {
            Ok(mut port) => {
                if let Some(the_porting) = port.pop() {
                    Ok(the_porting)
                } else {
                    Err(SpikeError::ObjectNotFound)
                }
            }
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct FetchPortings {
    pub page: i64,
    pub per_page: i64,
}

impl Message for FetchPortings {
    type Result = Result<(Vec<models::porting::Porting>, i64), SpikeError>;
}

impl Handler<FetchPortings> for DbExecutor {
    type Result = Result<(Vec<models::porting::Porting>, i64), SpikeError>;
    fn handle(
        &mut self,
        msg: FetchPortings,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::portings::dsl::*;

        match portings
            .select(portings::all_columns())
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<models::porting::Porting>(
                &*self.pool.get()?,
            ) {
            Ok(port) => Ok(port),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct InsertPorting(pub models::porting::NewPorting);

impl Message for InsertPorting {
    type Result = Result<models::porting::Porting, SpikeError>;
}

impl Handler<InsertPorting> for DbExecutor {
    type Result = Result<models::porting::Porting, SpikeError>;
    fn handle(
        &mut self,
        msg: InsertPorting,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::portings::dsl::*;

        match diesel::insert_into(portings)
            .values(msg.0)
            .get_result(&self.pool.get()?)
        {
            Ok(inserted) => Ok(inserted),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct UpdatePorting {
    pub porting: models::porting::NewPorting,
    pub porting_id: i32,
}

impl Message for UpdatePorting {
    type Result = Result<models::porting::Porting, SpikeError>;
}

impl Handler<UpdatePorting> for DbExecutor {
    type Result = Result<models::porting::Porting, SpikeError>;
    fn handle(
        &mut self,
        msg: UpdatePorting,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::portings::dsl::*;

        match diesel::update(portings)
            .filter(porting_id.eq(&msg.porting_id))
            .set(&msg.porting)
            .get_result(&self.pool.get()?)
        {
            Ok(updated) => Ok(updated),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
