use kardashev_app_server_protocol::AuthMode;
use kardashev_protocol::openai_models::ModelPreset;
use kardashev_protocol::openai_models::ModelUpgrade;
use kardashev_protocol::openai_models::ReasoningEffort;
use kardashev_protocol::openai_models::ReasoningEffortPreset;
use once_cell::sync::Lazy;

static PRESETS: Lazy<Vec<ModelPreset>> = Lazy::new(|| {
    vec![
        ModelPreset {
            id: "nova".to_string(),
            model: "nova".to_string(),
            display_name: "Nova".to_string(),
            description: "High-performance model for complex coding tasks".to_string(),
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: vec![
                ReasoningEffortPreset {
                    effort: ReasoningEffort::Low,
                    description: "Fast responses with lighter reasoning".to_string(),
                },
                ReasoningEffortPreset {
                    effort: ReasoningEffort::Medium,
                    description: "Balances speed and reasoning depth for everyday tasks".to_string(),
                },
                ReasoningEffortPreset {
                    effort: ReasoningEffort::High,
                    description: "Maximizes reasoning depth for complex problems".to_string(),
                },
            ],
            is_default: true,
            upgrade: None,
            show_in_picker: true,
        },
        ModelPreset {
            id: "nebula".to_string(),
            model: "nebula".to_string(),
            display_name: "Nebula".to_string(),
            description: "Optimized for code review and analysis".to_string(),
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: vec![
                ReasoningEffortPreset {
                    effort: ReasoningEffort::Low,
                    description: "Fast responses with lighter reasoning".to_string(),
                },
                ReasoningEffortPreset {
                    effort: ReasoningEffort::Medium,
                    description: "Balances speed and reasoning depth for everyday tasks".to_string(),
                },
                ReasoningEffortPreset {
                    effort: ReasoningEffort::High,
                    description: "Maximizes reasoning depth for complex problems".to_string(),
                },
            ],
            is_default: false,
            upgrade: None,
            show_in_picker: true,
        },
        ModelPreset {
            id: "pulsar".to_string(),
            model: "pulsar".to_string(),
            display_name: "Pulsar".to_string(),
            description: "Fast and efficient model for quick tasks".to_string(),
            default_reasoning_effort: ReasoningEffort::Low,
            supported_reasoning_efforts: vec![
                ReasoningEffortPreset {
                    effort: ReasoningEffort::Low,
                    description: "Fast responses for quick tasks".to_string(),
                },
                ReasoningEffortPreset {
                    effort: ReasoningEffort::Medium,
                    description: "Moderate reasoning for standard tasks".to_string(),
                },
            ],
            is_default: false,
            upgrade: None,
            show_in_picker: true,
        },
    ]
});

pub(crate) fn builtin_model_presets(_auth_mode: Option<AuthMode>) -> Vec<ModelPreset> {
    PRESETS
        .iter()
        .filter(|preset| preset.show_in_picker)
        .cloned()
        .collect()
}

// todo(aibrahim): remove this once we migrate tests
pub fn all_model_presets() -> &'static Vec<ModelPreset> {
    &PRESETS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_default_model_is_configured() {
        let default_models = PRESETS.iter().filter(|preset| preset.is_default).count();
        assert!(default_models == 1);
    }
}
