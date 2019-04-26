pub mod mna;
pub mod number;
pub mod pagination;
pub mod rsp;
pub mod token;
pub mod user;

use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExecutor {
    pool: Pool<ConnectionManager<PgConnection>>,
}

unsafe impl Send for DbExecutor {}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(db_url: &str) -> DbExecutor {
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::builder().build(manager);
        DbExecutor {
            pool: pool.expect(&format!(
                "Error creating connection pool to {}",
                db_url
            )),
        }
    }
}
