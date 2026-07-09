// src/auth/oauth/google.rs
use crate::errors::AppError;
use crate::auth::oauth::models::{GoogleUserInfo, OAuthUserProfile};
use reqwest::Client;

pub async fn verify_id_token(token: &str) -> Result<OAuthUserProfile, AppError> {
    // Google provides a tokeninfo endpoint for validating an ID token
    let url = format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        token
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("Google API request failed: {}", e)))?;

    // Check the status code
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::AuthenticationError(format!("Google token verification failed: {}", error_text)));
    }

    // Parse the user info
    let user_info: GoogleUserInfo = response
        .json()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("Failed to parse Google user info: {}", e)))?;

    // Verify the email is verified (optional)
    if let Some(verified) = user_info.email_verified {
        if !verified {
            return Err(AppError::AuthenticationError("Google email is not verified".to_string()));
        }
    }

    // Convert to the generic user profile format
    Ok(OAuthUserProfile {
        provider: "google".to_string(),
        provider_user_id: user_info.sub,
        email: user_info.email,
        name: user_info.name,
        picture: user_info.picture,
    })
}