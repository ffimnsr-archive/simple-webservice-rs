extern crate uuid;
extern crate chrono;

use self::uuid::Uuid;
use self::chrono::{DateTime, Utc};

use super::schema::bug_reports;

#[derive(Queryable)]
pub struct BugReport {
  pub id: Uuid,
  pub title: String,
  pub organization_affected: String,
  pub content: String,
  pub wallet_address: String,
  pub feedback: String,
  pub status: i16,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="bug_reports"]
pub struct NewBugReport<'a> {
  pub title: &'a str,
  pub organization_affected: &'a str,
  pub content: &'a str,
  pub wallet_address: &'a str,
}

