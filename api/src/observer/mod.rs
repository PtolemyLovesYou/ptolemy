use crate::state::ApiAppState;
use ptolemy::generated::observer::{
    observer_authentication_server::ObserverAuthenticationServer, observer_server::ObserverServer,
};
use service::{MyObserver, MyObserverAuthentication};

pub mod records;
pub mod service;

pub async fn observer_service(state: ApiAppState) -> ObserverServer<MyObserver> {
    let service = self::service::MyObserver::new(state.clone()).await;

    ObserverServer::new(service)
}

pub async fn authentication_service(state: ApiAppState) -> ObserverAuthenticationServer<MyObserverAuthentication> {
    let service = self::service::MyObserverAuthentication::new(state.clone()).await;

    ObserverAuthenticationServer::new(service)
}
