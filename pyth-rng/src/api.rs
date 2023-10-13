use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
};

use std::sync::{
Arc,
};

mod get_randomness_proof;

use crate::PebbleHashChain;

pub use {
    get_randomness_proof::*,
};


#[derive(Clone)]
pub struct ApiState {
    pub state: Arc<PebbleHashChain>,
}

// FIXME: real errors
pub enum RestError {
    TestError,
}

impl IntoResponse for RestError {
    fn into_response(self) -> Response {
        match self {
            RestError::TestError => {
                (StatusCode::NOT_FOUND, "Update data not found").into_response()
            }
        }
    }
}

