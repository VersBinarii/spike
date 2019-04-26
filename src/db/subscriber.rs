use actix::prelude::*;
use diesel::prelude::*;

use crate::db::pagination::*;
use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

pub struct FetchSubscriber(pub i32);

impl Message for FetchSubscriber {
    type Result = Result<models::Subscriber, SpikeError>;
}

impl Handler<FetchSubscriber> for DbExecutor {
    type Result = Result<models::Subscriber, SpikeError>;

    fn handle(
        &mut self,
        msg: FetchSubscriber,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::subscribers::dsl::*;

        match subscribers
            .filter(subscriber_id.eq(msg.0))
            .limit(1)
            .load::<models::Subscriber>(&self.pool.get()?)
        {
            Ok(mut ret_rsp) => {
                if let Some(the_rsp) = ret_rsp.pop() {
                    Ok(the_rsp)
                } else {
                    Err(SpikeError::InvalidId)
                }
            }
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct FetchSubscribers {
    pub page: i64,
    pub per_page: i64,
}

impl Message for FetchSubscribers {
    type Result = Result<(Vec<models::Subscriber>, i64), SpikeError>;
}

impl Handler<FetchSubscribers> for DbExecutor {
    type Result = Result<(Vec<models::Subscriber>, i64), SpikeError>;
    fn handle(
        &mut self,
        msg: FetchSubscribers,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::subscribers::dsl::*;

        match subscribers
            .select(subscribers::all_columns())
            .paginate(msg.page)
            .per_page(msg.per_page)
            .load_and_count_pages::<models::Subscriber>(&*self.pool.get()?)
        {
            Ok(subs) => Ok(subs),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct InsertSubscriber(pub models::NewSubscriber);

impl Message for InsertSubscriber {
    type Result = Result<models::Subscriber, SpikeError>;
}

impl Handler<InsertSubscriber> for DbExecutor {
    type Result = Result<models::Subscriber, SpikeError>;
    fn handle(
        &mut self,
        msg: InsertSubscriber,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::subscribers::dsl::*;

        match diesel::insert_into(subscribers)
            .values(msg.0)
            .get_result(&self.pool.get()?)
        {
            Ok(inserted_sub) => Ok(inserted_sub),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct UpdateSubscriber {
    pub subscriber: models::NewSubscriber,
    pub subscriber_id: i32,
}

impl Message for UpdateSubscriber {
    type Result = Result<models::Subscriber, SpikeError>;
}

impl Handler<UpdateSubscriber> for DbExecutor {
    type Result = Result<models::Subscriber, SpikeError>;
    fn handle(
        &mut self,
        msg: UpdateSubscriber,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::subscribers::dsl::*;

        match diesel::update(subscribers)
            .filter(subscriber_id.eq(&msg.subscriber_id))
            .set(&msg.subscriber)
            .get_result(&self.pool.get()?)
        {
            Ok(updated_sub) => Ok(updated_sub),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
