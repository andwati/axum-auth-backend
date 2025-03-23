use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Serialize, Deserialize, Validate, Debug, Default, Clone)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    #[validate(
        length(min = 1, message = "Confirm  Password is required"),
        must_match(other = "password", message = "passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Default, Clone)]
pub struct RequestQueryDto {
    #[validate(range(min = 1, message = "Page must be greater than 0"))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50, message = "Limit must be greater than 0"))]
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            verified: user.verified,
            role: user.role.to_str().to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub fn filter_users(user: &[User]) -> Vec<FilterUserDto> {
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Default, Clone)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct RoleUpdateDto {
    #[validate(custom = "validate_user_role")]
    pub role: UserRole,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        _ => Err(validator::ValidationError::new("Invalid role")),
    }
}

#[derive(Serialize, Deserialize, Validate, Debug, Default, Clone)]
pub struct UserPasswordUpdateDto {
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub new_password: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters"),
        must_match(other = "new_password", message = "passwords do not match")
    )]
    pub new_password_confirm: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub old_password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct ForgotPasswordRequestDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
    #[validate(length(min = 6, message = "New Password must be at least 6 characters"))]
    pub new_password: String,
    #[validate(
        length(
            min = 6,
            message = "Confirm New Password must be at least 6 characters"
        ),
        must_match(other = "new_password", message = "new passwords do not match")
    )]
    pub new_password_confirm: String,
}
