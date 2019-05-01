use actix::prelude::*;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;

#[derive(Debug)]
pub struct FetchUser {
    pub username: String,
}

impl Message for FetchUser {
    type Result = Result<models::User, SpikeError>;
}

impl Handler<FetchUser> for DbExecutor {
    type Result = Result<models::User, SpikeError>;

    fn handle(
        &mut self,
        msg: FetchUser,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::users::dsl::*;
        match users
            .filter(username.eq(msg.username))
            .limit(1)
            .load::<models::User>(&self.pool.get()?)
        {
            Ok(mut fetched_user) => {
                if let Some(the_user) = fetched_user.pop() {
                    Ok(the_user)
                } else {
                    Err(SpikeError::ObjectNotFound)
                }
            }
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}
