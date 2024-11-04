use axum::{Extension, Json, Router};
use axum::routing::post;
use axum_valid::Valid;

use domain::errors::ApiResult;
use domain::services::user_service::DynUserService;

use domain::services::service_registry::ServiceRegistry;

use crate::models::requests::{LoginRequest, CreateUserRequest};
use crate::models::responses::{LoginResponse, CreateUserResponse};
pub struct UserController;

impl UserController {
    pub fn new_router(service_registry: ServiceRegistry) -> Router {
        Router::new()
            .route("/login", post(Self::user_login))   
            .route("/signup", post(Self::user_signup))  
            .layer(Extension(service_registry.user_service))
    }

    pub async fn user_login(
        Extension(user_service): Extension<DynUserService>,
        Valid(Json(request)): Valid<Json<LoginRequest>>,
    ) -> ApiResult<Json<LoginResponse>> {
        Ok(Json(LoginResponse {
            token: user_service.user_login(&request.id, &request.password).await?,
        }))
    }

    pub async fn user_signup(
        Extension(user_service): Extension<DynUserService>,
        Valid(Json(request)): Valid<Json<CreateUserRequest>>,
    ) -> ApiResult<Json<CreateUserResponse>> {
        Ok(Json(CreateUserResponse {
            user: user_service.user_signup(&request.id, &request.password).await?,
        }))
    }
}
