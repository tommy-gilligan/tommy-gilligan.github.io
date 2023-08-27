use hyper::{body::Incoming as IncomingBody, Request, Response};
use hyper_staticfile::Static;

use std::{future::Future, path::PathBuf, pin::Pin};

pub struct Service {
    inner: Static,
}

impl Service {
    #[must_use]
    pub fn new(output: PathBuf) -> Self {
        Self {
            inner: hyper_staticfile::Static::new(output),
        }
    }
}

impl hyper::service::Service<Request<IncomingBody>> for Service {
    type Response = Response<hyper_staticfile::Body>;
    type Error = std::io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        Box::pin(self.inner.clone().serve(req))
    }
}
