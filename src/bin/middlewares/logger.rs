use futures::{future, Future};
use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::{request_id, FromState, State};
use hyper::{Method, Uri};

#[derive(Clone, NewMiddleware)]
pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        info!("[{}] pre chain", request_id(&state));
        {
            let method = Method::borrow_from(&state);
            let uri = Uri::borrow_from(&state);
            info!("Method: {:?}", method);
            info!("URI: {:?}", uri);
        }

        let f = chain(state).and_then(move |(state, response)| {
            {
                info!("[{}] post chain", request_id(&state));
            }
            future::ok((state, response))
        });

        Box::new(f)
    }
}
