//! Botgo-compatible OpenAPI extension points.

#![allow(non_snake_case)]

use std::collections::HashSet;
use std::sync::{Arc, LazyLock, RwLock};

use reqwest::{Method, StatusCode, header::HeaderMap};

/// Request/response view passed to OpenAPI filters.
#[derive(Debug, Clone, Default)]
pub struct FilterContext {
    pub method: Option<Method>,
    pub url: Option<String>,
    pub request_headers: HeaderMap,
    pub response_status: Option<StatusCode>,
    pub response_headers: HeaderMap,
}

impl FilterContext {
    pub fn new(request_headers: HeaderMap, response_headers: HeaderMap) -> Self {
        Self {
            method: None,
            url: None,
            request_headers,
            response_status: None,
            response_headers,
        }
    }

    pub fn request(method: Method, url: impl Into<String>, request_headers: HeaderMap) -> Self {
        Self {
            method: Some(method),
            url: Some(url.into()),
            request_headers,
            response_status: None,
            response_headers: HeaderMap::new(),
        }
    }

    pub fn response(
        method: Method,
        url: impl Into<String>,
        request_headers: HeaderMap,
        response_status: StatusCode,
        response_headers: HeaderMap,
    ) -> Self {
        Self {
            method: Some(method),
            url: Some(url.into()),
            request_headers,
            response_status: Some(response_status),
            response_headers,
        }
    }
}

/// Botgo-compatible HTTP filter callback.
pub type HTTPFilter = Arc<dyn Fn(&mut FilterContext) -> crate::Result<()> + Send + Sync + 'static>;

#[derive(Default)]
struct FilterChain {
    seen: HashSet<String>,
    filters: Vec<HTTPFilter>,
}

impl FilterChain {
    fn register(&mut self, name: impl Into<String>, filter: HTTPFilter) {
        let name = name.into();
        if self.seen.insert(name) {
            self.filters.push(filter);
        }
    }

    fn snapshot(&self) -> Vec<HTTPFilter> {
        self.filters.clone()
    }
}

fn run_filters(filters: Vec<HTTPFilter>, context: &mut FilterContext) -> crate::Result<()> {
    for filter in filters {
        filter(context)?;
    }
    Ok(())
}

static REQ_FILTERS: LazyLock<RwLock<FilterChain>> =
    LazyLock::new(|| RwLock::new(FilterChain::default()));
static RESP_FILTERS: LazyLock<RwLock<FilterChain>> =
    LazyLock::new(|| RwLock::new(FilterChain::default()));

pub fn RegisterReqFilter(
    name: impl Into<String>,
    filter: impl Fn(&mut FilterContext) -> crate::Result<()> + Send + Sync + 'static,
) {
    if let Ok(mut filters) = REQ_FILTERS.write() {
        filters.register(name, Arc::new(filter));
    }
}

pub fn RegisterRespFilter(
    name: impl Into<String>,
    filter: impl Fn(&mut FilterContext) -> crate::Result<()> + Send + Sync + 'static,
) {
    if let Ok(mut filters) = RESP_FILTERS.write() {
        filters.register(name, Arc::new(filter));
    }
}

pub fn DoReqFilterChains(context: &mut FilterContext) -> crate::Result<()> {
    let filters = REQ_FILTERS
        .read()
        .map_err(|_| crate::BotError::internal("request filter chain lock poisoned"))?
        .snapshot();
    run_filters(filters, context)
}

pub fn DoRespFilterChains(context: &mut FilterContext) -> crate::Result<()> {
    let filters = RESP_FILTERS
        .read()
        .map_err(|_| crate::BotError::internal("response filter chain lock poisoned"))?
        .snapshot();
    run_filters(filters, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn filters_run_in_registration_order_and_skip_duplicate_names() {
        RegisterReqFilter("test-order-a", |context| {
            context
                .request_headers
                .insert("x-filter-order", HeaderValue::from_static("a"));
            Ok(())
        });
        RegisterReqFilter("test-order-b", |context| {
            let value = context
                .request_headers
                .get("x-filter-order")
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_string()
                + "b";
            context.request_headers.insert(
                "x-filter-order",
                HeaderValue::from_str(&value).expect("valid header value"),
            );
            Ok(())
        });
        RegisterReqFilter("test-order-b", |_| {
            panic!("duplicate filter should be ignored")
        });

        let mut context = FilterContext::default();
        DoReqFilterChains(&mut context).unwrap();
        assert_eq!(
            context
                .request_headers
                .get("x-filter-order")
                .and_then(|value| value.to_str().ok()),
            Some("ab")
        );
    }
}
