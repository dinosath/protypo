use loco_rs::{controller::bad_request, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{models::{entities::user}};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub username: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserDetail,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserDetail {
    pub username: String,
    pub last_login: String,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &user::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            user: UserDetail {
                username: user.username.to_string(),
                last_login: "n/a".to_string(),
            },
        }
    }
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let res = user::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            let msg = "could not register user";

            tracing::info!(message = err.to_string(), user_email = &params.email, msg,);
            return bad_request(msg);
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;
    format::json(&user)
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyParams>,
) -> Result<Response> {
    let user = user::Model::find_by_verification_token(&ctx.db, &params.token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(username = user.username.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(username = user.username.to_string(), "user verified");
    }

    format::empty_json()
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = user::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::empty_json();
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    format::empty_json()
}

/// reset user password by the given parameters
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = user::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::empty_json();
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::empty_json()
}

/// Creates a user login and returns a token
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    let user = user::Model::find_by_username(&ctx.db, &params.username).await?;

    let valid = user.verify_password(&params.password);

    //TODO check validation error
    // if !valid {
    //     return unauthorized("unauthorized!");
    // }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("auth")
        .add("/register", post(register))
        .add("/verify", post(verify))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
}
