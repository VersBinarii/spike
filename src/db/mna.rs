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
                    Err(SpikeError::InvalidId)
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
    type Result = Result<Vec<models::Mna>, SpikeError>;
}

impl Handler<FetchMnas> for DbExecutor {
    type Result = Result<Vec<models::Mna>, SpikeError>;

    fn handle(
        &mut self,
        msg: FetchMnas,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::mna::dsl::*;
        use std::ops::Deref;

        match mna
            .select(mna::all_columns())
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<models::Mna>(&self.pool.get()?.deref())
        {
            Ok(mnas) => Ok(mnas.0),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
