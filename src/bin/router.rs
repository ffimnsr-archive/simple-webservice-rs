use dotenv::dotenv;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use hyper::{Get, Head};
use std::env;

use super::handlers::*;
use super::middlewares::logger::*;
use super::middlewares::diesel_pool::*;

pub fn router() -> Router {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
      .expect("database url must be set");

    let (chain, pipelines) = single_pipeline(new_pipeline()
        .add(LoggerMiddleware)
        .add(DieselMiddleware::new(&*database_url))
        .build());

    build_router(chain, pipelines, |route| {
        route.request(vec![Get, Head], "/").to(index);

        route.scope("/api", |route| {
            route.get("/bug-reports").to(api::bug_reports::index);

            route.associate("/bug-report", |assoc| {
                assoc.get()
                    .with_query_string_extractor::<api::bug_reports::QueryStringExtractor>()
                    .to(api::bug_reports::get);

                assoc.post().to(api::bug_reports::post);
                assoc.put().to(index);
                assoc.patch().to(index);
                assoc.delete().to(api::bug_reports::get);
            });
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::StatusCode;
    use gotham::test::TestServer;

    #[test]
    fn index_get() {
        let test_server = TestServer::new(|| Ok(index)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"index");
    }
}
