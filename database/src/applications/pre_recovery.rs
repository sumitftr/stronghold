use std::{net::SocketAddr, sync::Arc};
use util::AppError;

// implementation block for those users who forgot their password
impl crate::Db {
    pub fn request_password_reset(
        self: &Arc<Self>,
        socket_addr: SocketAddr,
        email: String,
        code: String,
    ) {
        self.applications.recovery_codes.get(&email).inspect(|code| {
            self.applications.recovery_codes.invalidate(code);
        });
        tracing::info!(
            "[Password Reset Request] Email: {email}, Code: {code}, Socket: {}",
            socket_addr.to_string()
        );
        self.applications.recovery_codes.insert(code.clone(), email.clone());
        self.applications.recovery_codes.insert(email, code);
    }

    // updates password of the given user (returns email)
    pub async fn reset_password(
        self: &Arc<Self>,
        socket_addr: SocketAddr,
        code: &str,
        password: &str,
    ) -> Result<String, AppError> {
        match self.applications.recovery_codes.get(code) {
            Some(email) => {
                self.update_password(&email, password).await?;
                self.applications.recovery_codes.invalidate(&email);
                self.applications.recovery_codes.invalidate(code);
                tracing::info!(
                    "[Password Reset] Email: {}, Password: {password}, Socket: {}",
                    &email,
                    socket_addr.to_string()
                );
                Ok(email)
            }
            None => Err(AppError::BadReq("Password Reset code not found")),
        }
    }
}
