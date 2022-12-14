#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum IpGetResponse {
    /// Get Global IPv4 address of the system
    GetGlobalIPv
    (models::IpGet200Response)
    ,
    /// Internal Server Error
    InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum IpPostResponse {
    /// The new IPv4 address has been registered
    TheNewIPv
    ,
    /// Bad Request
    BadRequest
    ,
    /// Internal Server Error
    InternalServerError
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    async fn ip_get(
        &self,
        context: &C) -> Result<IpGetResponse, ApiError>;

    async fn ip_post(
        &self,
        ip_get_request: Option<models::IpGetRequest>,
        context: &C) -> Result<IpPostResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    async fn ip_get(
        &self,
        ) -> Result<IpGetResponse, ApiError>;

    async fn ip_post(
        &self,
        ip_get_request: Option<models::IpGetRequest>,
        ) -> Result<IpPostResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    async fn ip_get(
        &self,
        ) -> Result<IpGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().ip_get(&context).await
    }

    async fn ip_post(
        &self,
        ip_get_request: Option<models::IpGetRequest>,
        ) -> Result<IpPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().ip_post(ip_get_request, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
