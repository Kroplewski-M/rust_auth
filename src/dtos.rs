use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::models::User;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Username must be between 1 and 100 characters"
    ))]
    pub name: String,
    #[validate(
        email(message = "Email is not valid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,
    #[validate(length(
        min = 5,
        max = 100,
        message = "Password must be between 5 and 100 characters"
    ))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    #[serde(rename = "passwordConfirmation")]
    pub password_confirmation: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginUserDto {
    #[validate(
        email(message = "Email is not valid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, IntoParams)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            role: user.role.to_str().to_string(),
            photo: user.photo.to_owned(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
    pub fn filter_users(users: &[User]) -> Vec<Self> {
        users.iter().map(Self::filter_user).collect()
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListResponseDto {
    pub status: String,
    pub data: Vec<FilterUserDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
