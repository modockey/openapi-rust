//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::http::request;
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls())
                .expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM)
                .expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem")
                .expect("Failed to set certificate chain");
            ssl.check_private_key()
                .expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr)
            .serve(service)
            .await
            .unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server {
            marker: PhantomData,
        }
    }
}

use openapi_client::server::MakeService;
use openapi_client::IpGetResponse::GetGlobalIPv;
use openapi_client::IpPostResponse::*;
use openapi_client::{Api, IpGetResponse, IpPostResponse};
use std::error::Error;
use swagger::ApiError;

use crate::db;
use crate::db::model::schema::ipv4_history::ipv4_address;
use crate::usecase;
use usecase::*;

use models::IpGet200Response;

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    async fn ip_get(&self, context: &C) -> Result<IpGetResponse, ApiError> {
        let context = context.clone();
        info!("get_ip() - X-Span-ID: {:?}", context.get().0.clone());
        match get_effective_ipv4_record() {
            Ok(ipv4_record) => Ok(GetGlobalIPv(IpGet200Response {
                ipv4_address: Some(ipv4_record.ipv4_address.to_string()),
                checked_at: Some(ipv4_record.last_checked_at),
            })),
            Err(e) => Err(ApiError(e.into())),
        }
    }

    async fn ip_post(
        &self,
        ip_get_request: Option<models::IpGetRequest>,
        context: &C,
    ) -> Result<IpPostResponse, ApiError> {
        let context = context.clone();
        info!(
            "ip_post({:?}) - X-Span-ID: {:?}",
            ip_get_request,
            context.get().0.clone()
        );

        if let Some(request) = ip_get_request && let Some(address)=request.ipv4_address && is_ipv4(&address){
            match post_ip4_address(&address) {
        Ok(()) => Ok(TheNewIPv {}),
                Err(e) => Err(ApiError(e.into())),
            }
        } else {
                return Ok(BadRequest);
        }
    }
}

use regex::Regex;

fn is_ipv4(text: &str) -> bool {
    let re = Regex::new(
        r"^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$",
    ).unwrap();
    return re.is_match(text);
}
