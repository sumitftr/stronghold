use super::{RegistrantEntry, RegistrantStatus};
use crate::users::User;
use sqlx::types::time::OffsetDateTime;
use std::{net::SocketAddr, sync::Arc};
use util::AppError;

// sub steps for registering an user
impl crate::Db {
    pub async fn create_registrant(
        self: &Arc<Self>,
        socket: SocketAddr,
        name: String,
        email: String,
        otp: String,
    ) -> Result<(), AppError> {
        self.is_email_available(&email).await?;
        self.applications.insert_registrant(
            email,
            RegistrantEntry {
                socket_addr: socket,
                display_name: Some(name),
                password: None,
                icon: None,
                phone: None,
                oauth_provider: util::oauth::OAuthProvider::None,
                status: RegistrantStatus::Created(otp),
            },
        );
        Ok(())
    }

    pub async fn update_registrant_otp(
        self: &Arc<Self>,
        email: &str,
        otp: String,
    ) -> Result<(), AppError> {
        if let Some(mut entry) = self.applications.registrants.get(email) {
            entry.status = RegistrantStatus::Created(otp);
            self.applications.insert_registrant(email.to_string(), entry);
            Ok(())
        } else {
            Err(AppError::UserNotFound)
        }
    }

    pub async fn verify_registrant_email(
        self: &Arc<Self>,
        email: &str,
        otp: &str,
    ) -> Result<(), AppError> {
        let entry = self.applications.registrants.get(email).ok_or(AppError::UserNotFound)?;
        match &entry.status {
            RegistrantStatus::Created(db_otp) if db_otp == otp => {
                self.applications.insert_registrant(email.to_string(), entry);
                Ok(())
            }
            RegistrantStatus::Created(_) => Err(AppError::InvalidOTP),
            _ => Err(AppError::BadReq("Please verify the email")),
        }
    }

    pub async fn set_registrant_password(
        self: &Arc<Self>,
        email: &str,
        password: String,
    ) -> Result<(), AppError> {
        if let Some(mut entry) = self.applications.registrants.get(email) {
            entry.password = Some(password);
            self.applications.insert_registrant(email.to_string(), entry);
            Ok(())
        } else {
            Err(AppError::UserNotFound)
        }
    }

    // it works for both registration and openidconnect
    pub async fn set_registrant_username(
        self: &Arc<Self>,
        email: String,
        username: String,
    ) -> Result<User, AppError> {
        self.is_username_available(&username).await?;
        let mut registrant =
            self.applications.registrants.get(&email).ok_or(AppError::UserNotFound)?;

        let id = sqlx::types::Uuid::new_v4();
        // creating a new object in the bucket from the cdn url
        if registrant.icon.is_some() {
            let cdn_icon_url = registrant.icon.unwrap();
            // Download the image from the source URL
            let response = reqwest::get(&cdn_icon_url).await.map_err(|e| {
                tracing::error!("Failed to download image from {cdn_icon_url}: {e:#?}");
                AppError::ServerError
            })?;

            if !response.status().is_success() {
                tracing::error!(
                    "Failed to download image from {cdn_icon_url}: status {}",
                    response.status()
                );
                return Err(AppError::ServerError);
            }

            // Get the image data as bytes
            let data = response.bytes().await.map_err(|e| {
                tracing::error!("Failed to read image bytes from {}: {e:#?}", cdn_icon_url);
                AppError::ServerError
            })?;

            let filename =
                cdn_icon_url.split('/').next_back().unwrap().split('=').next().unwrap().to_owned();
            registrant.icon = Some(self.upload_icon(data, filename, &id.to_string()).await?);
        }

        let user = User {
            id,
            display_name: registrant.display_name.unwrap(),
            email,
            birth_date: None,
            password: registrant.password,
            username,
            banner: None,
            icon: registrant.icon,
            bio: None,
            legal_name: None,
            gender: None,
            phone: None,
            country: None,
            oauth_provider: registrant.oauth_provider,
            created: OffsetDateTime::now_utc(),
        };
        self.create_user_forced(&user).await;
        self.applications.remove_registrant(&user.email);
        Ok(user)
    }
}
