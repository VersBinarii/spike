use actix::prelude::*;
use diesel::prelude::*;

use crate::db::pagination::*;
use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

pub struct FetchMna(pub i32);

impl Message for FetchMna {
    type Result = Result<models::Mna, SpikeError>;
}

impl Handler<FetchMna> for DbExecutor {
    type Result = Result<models::Mna, SpikeError>;

    fn handle(&mut self, msg: FetchMna, _: &mut Self::Context) -> Self::Result {
        use crate::schema::mna::dsl::*;

        match mna
            .filter(mna_id.eq(msg.0))
            .limit(1)
            .load::<models::Mna>(&self.pool.get()?)
        {
            Ok(mut fetched_mna) => {
                if let Some(the_mna) = fetched_mna.pop() {
                    Ok(the_mna)
                } else {
                    Err(SpikeError::ObjectNotFound)
                }
            }
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct FetchMnas {
    pub page: i64,
    pub per_page: i64,
}

impl Message for FetchMnas {
    type Result = Result<(Vec<models::Mna>, i64), SpikeError>;
}

impl Handler<FetchMnas> for DbExecutor {
    type Result = Result<(Vec<models::Mna>, i64), SpikeError>;

    fn handle(
        &mut self,
        msg: FetchMnas,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::mna::dsl::*;

        match mna
            .select(mna::all_columns())
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<models::Mna>(&*self.pool.get()?)
        {
            Ok(mnas) => Ok(mnas),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct InsertMna(pub models::NewMna);

impl Message for InsertMna {
    type Result = Result<models::Mna, SpikeError>;
}

impl Handler<InsertMna> for DbExecutor {
    type Result = Result<models::Mna, SpikeError>;

    fn handle(
        &mut self,
        msg: InsertMna,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::mna::dsl::*;

        match diesel::insert_into(mna)
            .values(msg.0)
            .get_result(&self.pool.get()?)
        {
            Ok(inserted_mna) => Ok(inserted_mna),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct UpdateMna {
    pub mna: models::NewMna,
    pub mna_id: i32,
}

impl Message for UpdateMna {
    type Result = Result<models::Mna, SpikeError>;
}

impl Handler<UpdateMna> for DbExecutor {
    type Result = Result<models::Mna, SpikeError>;
    fn handle(
        &mut self,
        msg: UpdateMna,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::mna::dsl::*;

        match diesel::update(mna)
            .filter(mna_id.eq(&msg.mna_id))
            .set(&msg.mna)
            .get_result(&self.pool.get()?)
        {
            Ok(updated_mna) => Ok(updated_mna),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct ShowNumbersMna {
    pub mna_id: i32,
    pub page: i64,
    pub per_page: i64,
}

impl Message for ShowNumbersMna {
    type Result =
        Result<(Vec<(models::Mna, models::number::Number)>, i64), SpikeError>;
}

impl Handler<ShowNumbersMna> for DbExecutor {
    type Result =
        Result<(Vec<(models::Mna, models::number::Number)>, i64), SpikeError>;
    fn handle(
        &mut self,
        msg: ShowNumbersMna,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::mna::dsl::{mna, mna_id};
        use crate::schema::numbers::dsl::*;

        match mna
            .filter(mna_id.eq(&msg.mna_id))
            .inner_join(numbers)
            .order(mna_id.asc())
            .select((mna::all_columns(), numbers::all_columns()))
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<(models::Mna, models::number::Number)>(
                &*self.pool.get()?,
            ) {
            Ok(updated_mna) => Ok(updated_mna),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
