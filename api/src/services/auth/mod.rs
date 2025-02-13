use crate::state::ApiAppState;
use ptolemy::generated::observer::observer_authentication_server::ObserverAuthenticationServer;
use service::MyObserverAuthentication;

pub mod service;

pub async fn authentication_service(
    state: ApiAppState,
) -> ObserverAuthenticationServer<MyObserverAuthentication> {
    let service = self::service::MyObserverAuthentication::new(state.clone()).await;

    ObserverAuthenticationServer::new(service)
}
