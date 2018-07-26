#[allow(unused_imports)] // -- Future and Stream
use futures::{future, Future, Stream};
#[allow(unused_imports)] // -- IntoHandlerError
use gotham::handler::{HandlerFuture, IntoHandlerError, IntoResponse};
use gotham::http::response::create_response;
#[allow(unused_imports)] // -- FromState
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
use mime::{Mime, TEXT_PLAIN_UTF_8};
use serde_json;

header! { (OpenSesameHeader, "X-Open-Sesame-Meta") => [String] }

pub const WHALE: &'static str = r#"
▄██████████████▄▐█▄▄▄▄█▌
████████████████▌▀▀██▀▀
████▄████████████▄▄█▌
▄▄▄▄▄██████████████▀
"#;

macro_rules! generic_handler {
    ($($t:ident), *) => { $(
            pub fn $t(state: State) -> (State, Response) {
                let mut res = create_response(
                    &state,
                    StatusCode::Ok,
                    Some((String::from(WHALE).into_bytes(), TEXT_PLAIN_UTF_8)),
                );
                res.headers_mut().set(OpenSesameHeader("alice".to_owned()));

                (state, res)
            }
    )+ }
}

generic_handler!(index);

pub mod api {
    use super::*;
    pub mod bug_reports {
        use super::*;
        generic_handler!(index);

        #[derive(Deserialize, StateData, StaticResponseExtender)]
        pub struct QueryStringExtractor {
            name: String,
        }

        #[derive(Serialize)]
        pub struct Product {
            name: String,
        }

        impl IntoResponse for Product {
            fn into_response(self, state: &State) -> Response {
                let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
                let mut res = create_response(
                    state,
                    StatusCode::Ok,
                    Some((
                        serde_json::to_vec(&self).expect("serialize product"),
                        content_type,
                    )),
                );

                res.headers_mut().set(OpenSesameHeader("alice".to_owned()));
                res
            }
        }

        pub fn get(mut state: State) -> (State, Product) {
            let query_param = QueryStringExtractor::take_from(&mut state);
            let product = Product {
                name: query_param.name,
            };

            (state, product)
        }

        pub fn post(mut state: State) -> Box<HandlerFuture> {
            let f = Body::take_from(&mut state)
                .concat2()
                .then(|full_body| match full_body {
                    Ok(valid_body) => {
                        let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                        let content_type =
                            "application/json; charset=utf-8".parse::<Mime>().unwrap();
                        let mut res = create_response(
                            &state,
                            StatusCode::Ok,
                            Some((
                                body_content.into_bytes(),
                                content_type,
                            )),
                        );
                        res.headers_mut().set(OpenSesameHeader("alice".to_owned()));

                        future::ok((state, res))
                    }
                    Err(err) => return future::err((state, err.into_handler_error())),
                });

            Box::new(f)
        }
    }
}
