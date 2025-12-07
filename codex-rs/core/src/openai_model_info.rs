use crate::openai_models::model_family::ModelFamily;

// Shared constants for commonly used window/token sizes.
pub(crate) const CONTEXT_WINDOW_272K: i64 = 272_000;

/// Metadata about a model, particularly OpenAI models.
/// We may want to consider including details like the pricing for
/// input tokens, output tokens, etc., though users will need to be able to
/// override this in config.toml, as this information can get out of date.
/// Though this would help present more accurate pricing information in the UI.
#[derive(Debug)]
pub(crate) struct ModelInfo {
    /// Size of the context window in tokens. This is the maximum size of the input context.
    pub(crate) context_window: i64,

    /// Token threshold where we should automatically compact conversation history. This considers
    /// input tokens + output tokens of this turn.
    pub(crate) auto_compact_token_limit: Option<i64>,
}

impl ModelInfo {
    const fn new(context_window: i64) -> Self {
        Self {
            context_window,
            auto_compact_token_limit: Some(Self::default_auto_compact_limit(context_window)),
        }
    }

    const fn default_auto_compact_limit(context_window: i64) -> i64 {
        (context_window * 9) / 10
    }
}

pub(crate) fn get_model_info(model_family: &ModelFamily) -> Option<ModelInfo> {
    let slug = model_family.slug.as_str();
    match slug {
        // Cognisync models via OpenRouter - all have 128k context windows
        "nova" => Some(ModelInfo::new(128_000)),
        "nebula" => Some(ModelInfo::new(128_000)),
        "pulsar" => Some(ModelInfo::new(128_000)),

        _ => None,
    }
}
