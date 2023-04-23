use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::env;
use std::ops::{Deref, DerefMut};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::new(database_url);

    return Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Problems on building pool");
}

pub struct ConnectionPg(pub PooledConnection<ConnectionManager<PgConnection>>);
pub struct PostgresPool(pub PgPool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PostgresPool {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.rocket().state::<PgPool>() {
            Some(pool) => Outcome::Success(PostgresPool(pool.clone())),
            _ => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectionPg {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<PostgresPool>().await;

        match pool {
            Outcome::Success(pool) => {
                let connection = pool.0.get().expect("Error during get connection");
                Outcome::Success(ConnectionPg(connection))
            }
            Outcome::Failure(_) => Outcome::Failure((Status::InternalServerError, ())),
            Outcome::Forward(_) => Outcome::Forward(()),
        }
    }
}

impl Deref for ConnectionPg {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for ConnectionPg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
