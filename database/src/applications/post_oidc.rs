use super::{RegistrantEntry, RegistrantStatus};
use std::{net::SocketAddr, sync::Arc};
use util::AppError;

// sub steps for registering an user
impl crate::Db {
    pub async fn create_registrant_oidc(
        self: &Arc<Self>,
        socket_addr: SocketAddr,
        name: String,
        email: String,
        icon: String,
        oauth_provider: util::oauth::OAuthProvider,
    ) -> Result<(), AppError> {
        self.is_email_available(&email).await?;
        self.applications.insert_registrant(
            email,
            RegistrantEntry {
                socket_addr,
                display_name: Some(name),
                password: None,
                icon: Some(icon),
                phone: None,
                oauth_provider,
                status: RegistrantStatus::OpenIDConnected,
            },
        );
        Ok(())
    }
}
