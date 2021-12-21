#[macro_use]
extern crate error_chain;

mod errors;
mod app;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::{App, HttpServer, get, HttpResponse};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::web::Path;
use dotenv::dotenv;
use futures::future::{ok, Ready};
use errors::*;
use error_chain::ChainedError;

async fn some_db_query(err: bool, value: bool) -> Result<Option<i32>> {
    if err {
        bail!("error");
    } else if value {
        Ok(Some(42))
    } else {
        Ok(None)
    }
}

#[get("/{err}/{value}")]
async fn some_endpoint(Path((err, value)): Path<(bool, bool)>) -> Result<HttpResponse> {
    let value = some_db_query(err, value)
        .await
        .chain_err(|| "Unable to perform db query")?
        .or_not_found()?;

    Ok(HttpResponse::Ok().body(value.to_string()))
}

pub struct LogError;

impl<S, B> Transform<S> for LogError
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = LogErrorMiddleware<S>;
    type InitError = ();
    type Future = Ready<std::result::Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LogErrorMiddleware { service })
    }
}

pub struct LogErrorMiddleware<S> {
    service: S,
}

impl<S, B> Service for LogErrorMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        self.service.poll_ready(ctx).map_err(|e| e.into())
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let future = self.service.call(req);

        Box::pin(async move {
            let result = future.await;

            result.map_err(|e| {
                log::error!("{}", e.display_chain());

                e.into()
            })
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(LogError)
            .service(some_endpoint)
    })
        .bind("127.0.0.1:8080")?
        .bind("localhost:8080")?
        .run()
        .await
}
