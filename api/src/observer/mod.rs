use self::interceptor::{ObserverAuthenticationInterceptor, ObserverInterceptor};
use crate::state::ApiAppState;
use ptolemy::generated::observer::{
    observer_authentication_server::ObserverAuthenticationServer, observer_server::ObserverServer,
};
use service::{MyObserver, MyObserverAuthentication};
use tonic::service::interceptor::InterceptedService;

pub mod claims;
pub mod interceptor;
pub mod records;
pub mod service;

type ObserverService = InterceptedService<ObserverServer<MyObserver>, ObserverInterceptor>;
type ObserverAuthenticationService = InterceptedService<
    ObserverAuthenticationServer<MyObserverAuthentication>,
    ObserverAuthenticationInterceptor,
>;

pub async fn observer_service(state: ApiAppState) -> ObserverService {
    let service = self::service::MyObserver::new(state.clone()).await;
    let interceptor = interceptor::ObserverInterceptor {
        state: state.clone(),
    };

    ObserverServer::with_interceptor(service, interceptor)
}

pub async fn authentication_service(state: ApiAppState) -> ObserverAuthenticationService {
    let service = self::service::MyObserverAuthentication::new(state.clone()).await;
    let interceptor = interceptor::ObserverAuthenticationInterceptor::new(state.clone());
    ObserverAuthenticationServer::with_interceptor(service, interceptor)
}
