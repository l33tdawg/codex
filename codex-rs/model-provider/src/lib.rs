mod amazon_bedrock;
mod auth;
mod bearer_auth_provider;
mod models_endpoint;
mod provider;
mod shared_state;

pub use amazon_bedrock::is_supported_amazon_bedrock_region;
pub use auth::AgentIdentitySessionFallback;
pub use auth::ProviderAuthScope;
pub use auth::ResolvedProviderAuth;
pub use auth::auth_provider_from_auth;
pub use auth::auth_provider_from_auth_manager;
pub use auth::unauthenticated_auth_provider;
pub use bearer_auth_provider::BearerAuthProvider;
pub use bearer_auth_provider::BearerAuthProvider as CoreAuthProvider;
pub use codex_model_provider_info::AMAZON_BEDROCK_PROVIDER_ID;
pub use codex_model_provider_info::AMAZON_BEDROCK_RUNTIME_PROVIDER_ID;
pub use codex_model_provider_info::CHATGPT_CODEX_BASE_URL;
pub use codex_protocol::account::ProviderAccount;
pub use provider::ModelProvider;
pub use provider::ModelProviderFuture;
pub use provider::ProviderAccountError;
pub use provider::ProviderAccountResult;
pub use provider::ProviderAccountState;
pub use provider::ProviderAuthRecoveryMessages;
pub use provider::ProviderCapabilities;
pub use provider::ProviderUnauthorizedRecovery;
pub use provider::RemoteCompactionSupport;
pub use provider::SharedModelProvider;
pub use provider::create_model_provider;

/// Resolve the model used for automatic approval review.
///
/// An explicit `approval_review_model` in the provider configuration takes
/// precedence over the provider's own default (the built-in
/// `codex-auto-review` slug, or a provider-specific ID such as the ones
/// Amazon Bedrock returns).
pub fn resolve_approval_review_model(provider: &dyn ModelProvider) -> String {
    provider
        .info()
        .approval_review_model
        .clone()
        .unwrap_or_else(|| provider.approval_review_preferred_model().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_model_provider_info::ModelProviderInfo;

    #[test]
    fn resolve_approval_review_model_prefers_provider_config() {
        let provider = create_model_provider(
            ModelProviderInfo {
                name: "deepseek".to_string(),
                approval_review_model: Some("deepseek-flash".to_string()),
                ..ModelProviderInfo::default()
            },
            /*auth_manager*/ None,
        );

        assert_eq!(
            resolve_approval_review_model(provider.as_ref()),
            "deepseek-flash"
        );
    }

    #[test]
    fn resolve_approval_review_model_falls_back_to_provider_default() {
        let provider =
            create_model_provider(ModelProviderInfo::default(), /*auth_manager*/ None);

        assert_eq!(
            resolve_approval_review_model(provider.as_ref()),
            provider.approval_review_preferred_model()
        );
    }
}
