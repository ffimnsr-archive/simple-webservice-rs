#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{BugReport, NewBugReport};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_bug_report<'a>(conn: &PgConnection,
    title: &'a str, organization_affected: &'a str, content: &'a str,
    wallet_address: &'a str) -> BugReport {

    use schema::bug_reports;

    let new_bug_report = NewBugReport {
        title: title,
        organization_affected: organization_affected,
        content: content,
        wallet_address: wallet_address,
    };

    diesel::insert_into(bug_reports::table)
        .values(&new_bug_report)
        .get_result(conn)
        .expect("error saving new bug report")
}

