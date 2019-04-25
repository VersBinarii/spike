use actix::prelude::*;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::models;
use crate::utils::SpikeError;
use uuid::{Uuid, UuidVersion};

pub struct InsertToken(pub String);

impl Message for InsertToken {
    type Result = Result<models::Token, SpikeError>;
}

impl Handler<InsertToken> for DbExecutor {
    type Result = Result<models::Token, SpikeError>;
    fn handle(
        &mut self,
        msg: InsertToken,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::tokens::dsl::*;

        let uuid_token = Uuid::new(UuidVersion::Random).unwrap();

        match diesel::insert_into(tokens)
            .values((
                token_id.eq(uuid_token.simple().to_string()),
                username.eq(msg.0),
                expiry.eq(diesel::dsl::sql("now() + '3 hour'")),
            ))
            .get_result(&self.pool.get()?)
        {
            Ok(inserted_token) => Ok(inserted_token),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct FetchToken(pub String);

impl Message for FetchToken {
    type Result = Result<models::Token, SpikeError>;
}

impl Handler<FetchToken> for DbExecutor {
    type Result = Result<models::Token, SpikeError>;
    fn handle(
        &mut self,
        msg: FetchToken,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::tokens::dsl::*;

        match tokens
            .filter(token_id.eq(msg.0))
            .get_result(&self.pool.get()?)
        {
            Ok(user_token) => Ok(user_token),
            Err(e) => Err(SpikeError::DatabaseQueryError(e)),
        }
    }
}

pub struct DeleteToken(pub String);

impl Message for DeleteToken {
    type Result = Result<usize, SpikeError>;
}

impl Handler<DeleteToken> for DbExecutor {
    type Result = Result<usize, SpikeError>;
    fn handle(
        &mut self,
        msg: DeleteToken,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::tokens::dsl::*;

        diesel::delete(tokens.filter(username.eq(msg.0)))
            .execute(&self.pool.get()?)
            .map_err(SpikeError::from)
    }
}
