use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use validator::Validate;

use crate::{state::AppState, web::error::ApiError};

use super::{dto::{FilteredUser, UpdateUserDto}, service, utils::{filter_user, filter_users}};

const API_TAG: &str = "Users";
const ADMIN_API_TAG: &str = "Admin";

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = API_TAG,
    
    params(
        ("id" = uuid::Uuid, Path, description = "User id")
    ),

    responses(
        (status = 200, description = "Successfully getting user by id", body = FilteredUser),
        (status = 500, description = "Internal server error")
    ),
    
    security(
        ("token" = [])
    )
)]
pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>
) -> Result<Json<FilteredUser>, ApiError> {
    match service::get_user_by_id(&state.db, id).await {
        Ok(user) => Ok(Json(filter_user(&user))),
        Err(error) => Err(error)
    }
}


#[utoipa::path(
    get,
    path = "/api/users",
    tag = ADMIN_API_TAG,

    responses(
        (status = 200, description = "Successfully getting users", body = [FilteredUser]),
        (status = 500, description = "Internal server error")
    ),

    security(
        ("token" = [])
    )
)]
pub async fn get_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FilteredUser>>, ApiError> {
    match service::get_users(&state.db).await {
        Ok(users) => Ok(Json(filter_users(&users))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    patch,
    path = "/api/users/{id}",
    tag = API_TAG,

    params(
        ("id" = uuid::Uuid, Path, description = "User id")
    ),

    request_body = UpdateUserDto,

    responses(
        (status = 200, description = "Successfully update user", body = FilteredUser),
        (status = 400, description = "bad_request"),
        (status = 404, description = "users.user_not_found"),
        (status = 500, description = "app.internal_server_error")
    ),

    security(
        ("token" = [])
    )
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdateUserDto>,
) -> Result<Json<FilteredUser>, ApiError> {
    match body.validate() {
        Ok(_) => (),
        Err(_) => return Err(ApiError::BodyParsingError("Cannot parse body".to_owned())),
    };

    match service::update_user(&state.db, id, body).await {
        Ok(updated_user) => Ok(Json(filter_user(&updated_user))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = ADMIN_API_TAG,

    params(
        ("id" = uuid::Uuid, Path, description = "User id")
    ),

    security(
        ("token" = [])
    )
)]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>
) -> Result<(), ApiError> {
    match service::delete_user(&state.db, id).await {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}