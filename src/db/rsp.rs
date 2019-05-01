use actix::prelude::*;
use diesel::prelude::*;

use crate::db::pagination::*;
use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

pub struct FetchRsp(pub i32);

impl Message for FetchRsp {
    type Result = Result<models::Rsp, SpikeError>;
}

impl Handler<FetchRsp> for DbExecutor {
    type Result = Result<models::Rsp, SpikeError>;

    fn handle(&mut self, msg: FetchRsp, _: &mut Self::Context) -> Self::Result {
        use crate::schema::rsp::dsl::*;

        match rsp
            .filter(rsp_id.eq(msg.0))
            .limit(1)
            .load::<models::Rsp>(&self.pool.get()?)
        {
            Ok(mut ret_rsp) => {
                if let Some(the_rsp) = ret_rsp.pop() {
                    Ok(the_rsp)
                } else {
                    Err(SpikeError::ObjectNotFound)
                }
            }
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct FetchRsps {
    pub page: i64,
    pub per_page: i64,
}

impl Message for FetchRsps {
    type Result = Result<(Vec<models::Rsp>, i64), SpikeError>;
}

impl Handler<FetchRsps> for DbExecutor {
    type Result = Result<(Vec<models::Rsp>, i64), SpikeError>;
    fn handle(
        &mut self,
        msg: FetchRsps,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::rsp::dsl::*;

        match rsp
            .select(rsp::all_columns())
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<models::Rsp>(&*self.pool.get()?)
        {
            Ok(rsps) => Ok(rsps),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct InsertRsp(pub models::NewRsp);

impl Message for InsertRsp {
    type Result = Result<models::Rsp, SpikeError>;
}

impl Handler<InsertRsp> for DbExecutor {
    type Result = Result<models::Rsp, SpikeError>;
    fn handle(
        &mut self,
        msg: InsertRsp,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::rsp::dsl::*;

        match diesel::insert_into(rsp)
            .values(msg.0)
            .get_result(&self.pool.get()?)
        {
            Ok(inserted_rsp) => Ok(inserted_rsp),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct UpdateRsp {
    pub rsp: models::NewRsp,
    pub rsp_id: i32,
}

impl Message for UpdateRsp {
    type Result = Result<models::Rsp, SpikeError>;
}

impl Handler<UpdateRsp> for DbExecutor {
    type Result = Result<models::Rsp, SpikeError>;
    fn handle(
        &mut self,
        msg: UpdateRsp,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::rsp::dsl::*;

        match diesel::update(rsp)
            .filter(rsp_id.eq(&msg.rsp_id))
            .set(&msg.rsp)
            .get_result(&self.pool.get()?)
        {
            Ok(updated_rsp) => Ok(updated_rsp),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
