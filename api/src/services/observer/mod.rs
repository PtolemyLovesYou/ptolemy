use crate::state::ApiAppState;
use ptolemy::generated::observer::observer_server::ObserverServer;
use service::MyObserver;

pub mod records;
pub mod service;

pub async fn observer_service(state: ApiAppState) -> ObserverServer<MyObserver> {
    let service = self::service::MyObserver::new(state.clone()).await;

    ObserverServer::new(service)
}
