use sea_orm::TransactionTrait;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde_json::json;
use uuid::Uuid;
use chrono::{offset::Local,FixedOffset};

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        {
            if insert {
                let mut this = self;
                if let ActiveValue::Set(ref password) = this.password {
                    let hashed_password = hash::hash_password(password).map_err(|e| DbErr::Custom(format!("Password hash failed: {:?}", e)))?;
                    this.password = ActiveValue::Set(hashed_password);
                }
                this.pid = ActiveValue::Set(Uuid::new_v4());
                this.api_key = ActiveValue::Set(format!("lo-{}", Uuid::new_v4()));
                Ok(this)
            } else {
                Ok(self)
            }
        }
    }
}


impl Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(query::condition().eq(Column::Email, email).build())
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided username
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_username(db: &DatabaseConnection, username: &str) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(query::condition().eq(Column::Username, username).build())
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }


    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                query::condition()
                    .eq(Column::EmailVerificationToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                query::condition()
                    .eq(Column::ResetToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let user = Entity::find()
            .filter(
                query::condition()
                    .eq(Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided api key
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                query::condition()
                    .eq(Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn create_with_password(
        db: &DatabaseConnection,
        params: &crate::controllers::auth::RegisterParams,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if Entity::find()
            .filter(
                query::condition()
                    .eq(Column::Email, &params.email)
                    .build(),
            )
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists {});
        }

        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let user = ActiveModel {
            pid: ActiveValue::Set(Uuid::new_v4()),
            api_key: ActiveValue::Set(format!("lo-{}", Uuid::new_v4())),
            email: ActiveValue::set(Some(params.email.to_string())),
            password: ActiveValue::set(password_hash),
            username: ActiveValue::set(params.username.to_string()),
            ..Default::default()
        }
            .insert(&txn)
            .await?;

        txn.commit().await?;

        Ok(user)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        Ok(jwt::JWT::new(secret).generate_token(
            expiration,
            self.pid.to_string(),
            Some(json!({"Roll": "Administrator"})),
        )?)
    }
}

impl ActiveModel {
    /// Validate user schema
    ///
    /// # Errors
    ///
    /// when the active model is not valid

    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
        self.email_verification_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.reset_sent_at = ActiveValue::set(Some(Local::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
        self.reset_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.email_verified_at = ActiveValue::set(Some(Local::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
        Ok(self.update(db).await?)
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<Model> {
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        self.reset_token = ActiveValue::Set(None);
        self.reset_sent_at = ActiveValue::Set(None);
        Ok(self.update(db).await?)
    }
}