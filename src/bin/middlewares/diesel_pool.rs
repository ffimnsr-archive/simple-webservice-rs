use diesel::PgConnection;
use futures::{future, Future};
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
#[allow(unused_imports)] // -- FromState, request_id
use gotham::state::{request_id, FromState, State};
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use std::io;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

#[allow(dead_code)]
pub type DieselConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DieselPool = Pool<ConnectionManager<PgConnection>>;

#[allow(dead_code)]
#[derive(StateData)]
pub struct DieselMiddlewareData {
    pool: DieselPool,
}

impl DieselMiddlewareData {
    fn new(pool: DieselPool) -> Self {
        DieselMiddlewareData { pool }
    }

    #[allow(dead_code)]
    fn get_db_conn(&self) -> DieselConnection {
        return self.pool.get().expect("failed to get db connection");
    }
}

pub struct DieselMiddleware {
    pool: AssertUnwindSafe<DieselPool>,
}

pub struct DieselMiddlewareImpl {
    pool: DieselPool,
}

impl DieselMiddleware {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::<ConnectionManager<PgConnection>>::new(manager)
            .expect("failed to create diesel pool");

        DieselMiddleware::with_pool(pool)
    }

    pub fn with_pool(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        DieselMiddleware {
            pool: AssertUnwindSafe(pool),
        }
    }
}

impl NewMiddleware for DieselMiddleware {
    type Instance = DieselMiddlewareImpl;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => Ok(DieselMiddlewareImpl { pool }),
            Err(_) => {
                error!("panic: pool clone causes a panic");
                eprintln!("panic: pool clone causes a panic");
                process::abort()
            }
        }
    }
}

impl Clone for DieselMiddleware {
    fn clone(&self) -> Self {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => DieselMiddleware {
                pool: AssertUnwindSafe(pool),
            },
            Err(_) => {
                error!("panic: pool clone causes a panic");
                eprintln!("panic: pool clone causes a panic");
                process::abort()
            }
        }
    }
}

impl Middleware for DieselMiddlewareImpl {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        state.put(DieselMiddlewareData::new(self.pool));

        let f = chain(state).and_then(move |(state, response)| future::ok((state, response)));

        Box::new(f)
    }
}
