#[macro_use]
extern crate diesel;
extern crate uuid;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use self::models::{BugReport, NewBugReport};

pub fn create_bug_report<'a>(conn: &PgConnection,
                             title: &'a str, organization_affected: &'a str,
                             content: &'a str, wallet_address: &'a str) -> BugReport {

    use schema::bug_reports;

    let new_bug_report = NewBugReport {
        title: title,
        organization_affected: organization_affected,
        content: content,
        wallet_address: wallet_address,
    };

    diesel::insert_into(bug_reports::table)
        .values(&new_bug_report)
        .get_result::<BugReport>(conn)
        .expect("error saving new bug report")
}

pub fn select_bug_report<'a>(conn: &PgConnection) -> Vec<BugReport> {
    use schema::bug_reports::dsl::*;

    let results = bug_reports.filter(status.eq(0))
        .load::<BugReport>(conn)
        .expect("error loading bug rerpots");

    results
}

pub fn update_bug_report<'a>(conn: &PgConnection, id: uuid::Uuid) -> BugReport {
    use schema::bug_reports::dsl::{bug_reports, status};

    diesel::update(bug_reports.find(id))
        .set(status.eq(1))
        .get_result::<BugReport>(conn)
        .expect(&format!("unable to find bug report {:?}", id))
}

pub fn delete_bug_report<'a>(conn: &PgConnection, pattern: &'a str) -> usize {
    use schema::bug_reports::dsl::*;

    diesel::delete(bug_reports.filter(title.like(pattern)))
        .execute(conn)
        .expect("error deleting bug report")
}
