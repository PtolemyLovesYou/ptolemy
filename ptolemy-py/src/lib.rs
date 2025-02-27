use client::client::bytes_to_df;
use pyo3::{exceptions::PyValueError, prelude::*};

pub mod client;
pub mod enums;
pub mod graphql;
pub mod models;
pub mod types;

use crate::{
    client::{client::PtolemyClient, server_handler::QueryEngine},
    enums::{api_key_permission, user_status, workspace_role},
    graphql::PyGraphQLClient,
    models::add_models_to_module,
};
use ptolemy::generated::query_engine::query_engine_client::QueryEngineClient;

#[pyfunction(signature=(base_url, token, query, batch_size=None, timeout_seconds=None))]
pub fn query(
    py: Python<'_>,
    base_url: String,
    token: String,
    query: String,
    batch_size: Option<u32>,
    timeout_seconds: Option<u32>,
) -> PyResult<Vec<Py<PyAny>>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let client = rt
        .block_on(QueryEngineClient::connect(base_url.clone()))
        .unwrap();

    let mut query_engine_client = QueryEngine {
        client,
        token: Some(token),
    };

    let result = rt
        .block_on(query_engine_client.query(query, batch_size, timeout_seconds))
        .map_err(|e| PyValueError::new_err(e.to_string()))?
        .into_iter();

    let mut data = Vec::new();

    for r in result {
        data.push(bytes_to_df(py, r)?);
    }
    Ok(data)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
pub fn _core<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<PtolemyClient>()?;
    m.add_class::<PyGraphQLClient>()?;
    add_models_to_module(py, m)?;
    m.add_function(wrap_pyfunction!(query, m)?)?;
    api_key_permission::add_enum_to_module(py, m)?;
    user_status::add_enum_to_module(py, m)?;
    workspace_role::add_enum_to_module(py, m)?;
    Ok(())
}
