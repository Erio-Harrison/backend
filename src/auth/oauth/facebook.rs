// src/auth/oauth/facebook.rs
use crate::errors::AppError;
use crate::auth::oauth::models::{FacebookUserInfo, OAuthUserProfile};
use reqwest::Client;

pub async fn verify_access_token(token: &str) -> Result<OAuthUserProfile, AppError> {
    let url = format!(
        "https://graph.facebook.com/me?fields=id,name,email,picture&access_token={}",
        token
    );
    
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("Facebook API request failed: {}", e)))?;

    // Check the status code
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::AuthenticationError(format!("Facebook token verification failed: {}", error_text)));
    }

    // Parse the user info
    let user_info: FacebookUserInfo = response
        .json()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("Failed to parse Facebook user info: {}", e)))?;

    // Ensure an email is present
    let email = user_info.email.ok_or_else(|| {
        AppError::AuthenticationError("Facebook account has no associated email".to_string())
    })?;

    // Get the avatar URL (if any)
    let picture = user_info
        .picture_data
        .and_then(|data| data.data)
        .and_then(|pic| pic.url);

    // Convert to the generic user profile format
    Ok(OAuthUserProfile {
        provider: "facebook".to_string(),
        provider_user_id: user_info.id,
        email,
        name: user_info.name,
        picture,
    })
}
