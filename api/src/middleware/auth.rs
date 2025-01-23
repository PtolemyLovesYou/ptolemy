// use crate::{
//     crud::auth::user_api_key::get_user_api_key_user,
//     error::AuthError,
//     models::{middleware::{AuthHeader, AuthContext}, auth::prelude::ToModel as _},
//     state::ApiAppState,
//     crypto::ClaimType,
// };
// use axum::{
//     body::Body,
//     extract::State,
//     http::{Request, StatusCode},
//     middleware::Next,
//     response::IntoResponse,
// };

// pub async fn jwt_middleware(
//     State(state): State<crate::state::ApiAppState>,
//     mut req: Request<axum::body::Body>,
//     next: Next,
// ) -> Result<impl IntoResponse, StatusCode> {
//     let token_data = match req.extensions().get::<AuthHeader>() {
//         Some(AuthHeader::JWT(token)) => token,
//         _ => {
//             req.extensions_mut().insert(AuthContext::Unauthorized(AuthError::MissingHeader));
//             return Ok(next.run(req).await);
//         }
//     };

//     let mut conn = match state.get_conn().await {
//         Ok(c) => c,
//         Err(e) => {
//             tracing::error!("Failed to get database connection: {:?}", e);
//             req.extensions_mut().insert(AuthContext::Unauthorized(AuthError::InternalServerError));
//             return Ok(next.run(req).await);
//         }
//     };

//     let ext = match token_data.claim_type() {
//         ClaimType::UserJWT => {
//             match crate::crud::auth::user::get_user(
//                 &mut conn,
//                 &token_data.sub(),
//             ).await {
//                 Ok(u) => AuthContext::UserJWT { user: u.to_model() },
//                 Err(_) => AuthContext::Unauthorized(AuthError::NotFoundError)
//             }
//         },
//         ClaimType::ServiceAPIKeyJWT => {
//             match crate::crud::auth::service_api_key::get_service_api_key_by_id(
//                 &mut conn,
//                 &token_data.sub(),
//             ).await {
//                 Ok(sk) => AuthContext::WorkspaceJWT {
//                     service_api_key_id: sk.id,
//                     workspace_id: sk.workspace_id,
//                     permissions: sk.permissions,
//                 },
//                 Err(_) => AuthContext::Unauthorized(AuthError::NotFoundError)
//             }
//         }
//     };

//     req.extensions_mut().insert(ext);

//     Ok(next.run(req).await)
// }

// pub async fn api_key_auth_middleware(
//     State(state): State<ApiAppState>,
//     mut req: Request<Body>,
//     next: Next,
// ) -> Result<impl IntoResponse, StatusCode> {
//     let api_key = match req.extensions().get::<AuthHeader>() {
//         Some(AuthHeader::ApiKey(api_key)) => api_key,
//         _ => {
//             req.extensions_mut().insert(AuthContext::Unauthorized(AuthError::MissingHeader));
//             return Ok(next.run(req).await);
//         }
//     };

//     let mut conn = state.get_conn_http().await?;

//     match get_user_api_key_user(
//         &mut conn,
//         &api_key,
//         &state.password_handler
//     ).await {
//         Ok(user) => {
//             req.extensions_mut().insert(AuthContext::UserApiKey {user: user.to_model()});
//         },
//         Err(_) => {
//             req.extensions_mut().insert(AuthContext::Unauthorized(AuthError::InvalidToken));
//         }
//     }

//     Ok(next.run(req).await)
// }
