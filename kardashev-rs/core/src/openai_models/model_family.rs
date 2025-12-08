use kardashev_protocol::config_types::Verbosity;
use kardashev_protocol::openai_models::ModelInfo;
use kardashev_protocol::openai_models::ReasoningEffort;

use crate::config::Config;
use crate::config::types::ReasoningSummaryFormat;
use crate::tools::handlers::apply_patch::ApplyPatchToolType;
use crate::truncate::TruncationPolicy;
use kardashev_protocol::openai_models::ConfigShellToolType;

/// The `instructions` field in the payload sent to a model should always start
/// with this content.
const BASE_INSTRUCTIONS: &str = include_str!("../../prompt.md");

const GPT_5_KARDASHEV_INSTRUCTIONS: &str = include_str!("../../gpt_5_kardashev_prompt.md");
const GPT_5_1_INSTRUCTIONS: &str = include_str!("../../gpt_5_1_prompt.md");
const GPT_5_1_KARDASHEV_MAX_INSTRUCTIONS: &str = include_str!("../../gpt-5.1-codex-max_prompt.md");

/// A model family is a group of models that share certain characteristics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelFamily {
    /// The full model slug used to derive this model family, e.g.
    /// "gpt-4.1-2025-04-14".
    pub slug: String,

    /// The model family name, e.g. "gpt-4.1". Note this should able to be used
    /// with [`crate::openai_model_info::get_model_info`].
    pub family: String,

    /// True if the model needs additional instructions on how to use the
    /// "virtual" `apply_patch` CLI.
    pub needs_special_apply_patch_instructions: bool,

    // Whether the `reasoning` field can be set when making a request to this
    // model family. Note it has `effort` and `summary` subfields (though
    // `summary` is optional).
    pub supports_reasoning_summaries: bool,

    // The reasoning effort to use for this model family when none is explicitly chosen.
    pub default_reasoning_effort: Option<ReasoningEffort>,

    // Define if we need a special handling of reasoning summary
    pub reasoning_summary_format: ReasoningSummaryFormat,

    /// Whether this model supports parallel tool calls when using the
    /// Responses API.
    pub supports_parallel_tool_calls: bool,

    /// Present if the model performs better when `apply_patch` is provided as
    /// a tool call instead of just a bash command
    pub apply_patch_tool_type: Option<ApplyPatchToolType>,

    // Instructions to use for querying the model
    pub base_instructions: String,

    /// Names of beta tools that should be exposed to this model family.
    pub experimental_supported_tools: Vec<String>,

    /// Percentage of the context window considered usable for inputs, after
    /// reserving headroom for system prompts, tool overhead, and model output.
    /// This is applied when computing the effective context window seen by
    /// consumers.
    pub effective_context_window_percent: i64,

    /// If the model family supports setting the verbosity level when using Responses API.
    pub support_verbosity: bool,

    // The default verbosity level for this model family when using Responses API.
    pub default_verbosity: Option<Verbosity>,

    /// Preferred shell tool type for this model family when features do not override it.
    pub shell_type: ConfigShellToolType,

    pub truncation_policy: TruncationPolicy,
}

impl ModelFamily {
    pub fn with_config_overrides(mut self, config: &Config) -> Self {
        if let Some(supports_reasoning_summaries) = config.model_supports_reasoning_summaries {
            self.supports_reasoning_summaries = supports_reasoning_summaries;
        }
        if let Some(reasoning_summary_format) = config.model_reasoning_summary_format.as_ref() {
            self.reasoning_summary_format = reasoning_summary_format.clone();
        }
        self
    }
    pub fn with_remote_overrides(mut self, remote_models: Vec<ModelInfo>) -> Self {
        for model in remote_models {
            if model.slug == self.slug {
                self.default_reasoning_effort = Some(model.default_reasoning_level);
                self.shell_type = model.shell_type;
            }
        }
        self
    }
}

macro_rules! model_family {
    (
        $slug:expr, $family:expr $(, $key:ident : $value:expr )* $(,)?
    ) => {{
        // defaults
        #[allow(unused_mut)]
        let mut mf = ModelFamily {
            slug: $slug.to_string(),
            family: $family.to_string(),
            needs_special_apply_patch_instructions: false,
            supports_reasoning_summaries: false,
            reasoning_summary_format: ReasoningSummaryFormat::None,
            supports_parallel_tool_calls: false,
            apply_patch_tool_type: None,
            base_instructions: BASE_INSTRUCTIONS.to_string(),
            experimental_supported_tools: Vec::new(),
            effective_context_window_percent: 95,
            support_verbosity: false,
            shell_type: ConfigShellToolType::Default,
            default_verbosity: None,
            default_reasoning_effort: None,
            truncation_policy: TruncationPolicy::Bytes(10_000),
        };

        // apply overrides
        $(
            mf.$key = $value;
        )*
        mf
    }};
}

/// Returns a `ModelFamily` for the given model slug.
/// Supports Kardashev models (Nova, Nebula, Pulsar).
pub fn find_family_for_model(slug: &str) -> ModelFamily {
    if slug.starts_with("nova") {
        // Nova: Primary high-performance model
        model_family!(
            slug, "nova",
            needs_special_apply_patch_instructions: true,
            supports_parallel_tool_calls: true,
            shell_type: ConfigShellToolType::ShellCommand,
            truncation_policy: TruncationPolicy::Bytes(10_000),
        )
    } else if slug.starts_with("nebula") {
        // Nebula: Review and analysis model
        model_family!(
            slug, "nebula",
            needs_special_apply_patch_instructions: true,
            supports_parallel_tool_calls: true,
            shell_type: ConfigShellToolType::ShellCommand,
            truncation_policy: TruncationPolicy::Bytes(10_000),
        )
    } else if slug.starts_with("pulsar") {
        // Pulsar: Faster, lighter model
        model_family!(
            slug, "pulsar",
            needs_special_apply_patch_instructions: true,
            supports_parallel_tool_calls: true,
            shell_type: ConfigShellToolType::ShellCommand,
            truncation_policy: TruncationPolicy::Bytes(10_000),
        )
    } else {
        derive_default_model_family(slug)
    }
}

fn derive_default_model_family(model: &str) -> ModelFamily {
    ModelFamily {
        slug: model.to_string(),
        family: model.to_string(),
        needs_special_apply_patch_instructions: false,
        supports_reasoning_summaries: false,
        reasoning_summary_format: ReasoningSummaryFormat::None,
        supports_parallel_tool_calls: false,
        apply_patch_tool_type: None,
        base_instructions: BASE_INSTRUCTIONS.to_string(),
        experimental_supported_tools: Vec::new(),
        effective_context_window_percent: 95,
        support_verbosity: false,
        shell_type: ConfigShellToolType::Default,
        default_verbosity: None,
        default_reasoning_effort: None,
        truncation_policy: TruncationPolicy::Bytes(10_000),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kardashev_protocol::openai_models::ClientVersion;
    use kardashev_protocol::openai_models::ModelVisibility;

    fn remote(slug: &str, effort: ReasoningEffort, shell: ConfigShellToolType) -> ModelInfo {
        ModelInfo {
            slug: slug.to_string(),
            display_name: slug.to_string(),
            description: Some(format!("{slug} desc")),
            default_reasoning_level: effort,
            supported_reasoning_levels: vec![effort],
            shell_type: shell,
            visibility: ModelVisibility::List,
            minimal_client_version: ClientVersion(0, 1, 0),
            supported_in_api: true,
            priority: 1,
        }
    }

    #[test]
    fn remote_overrides_apply_when_slug_matches() {
        let family = model_family!("gpt-4o-mini", "gpt-4o-mini");
        assert_ne!(family.default_reasoning_effort, Some(ReasoningEffort::High));

        let updated = family.with_remote_overrides(vec![
            remote(
                "gpt-4o-mini",
                ReasoningEffort::High,
                ConfigShellToolType::ShellCommand,
            ),
            remote(
                "other-model",
                ReasoningEffort::Low,
                ConfigShellToolType::UnifiedExec,
            ),
        ]);

        assert_eq!(
            updated.default_reasoning_effort,
            Some(ReasoningEffort::High)
        );
        assert_eq!(updated.shell_type, ConfigShellToolType::ShellCommand);
    }

    #[test]
    fn remote_overrides_skip_non_matching_models() {
        let family = model_family!(
            "codex-mini-latest",
            "codex-mini-latest",
            shell_type: ConfigShellToolType::Local
        );

        let updated = family.clone().with_remote_overrides(vec![remote(
            "other",
            ReasoningEffort::High,
            ConfigShellToolType::ShellCommand,
        )]);

        assert_eq!(
            updated.default_reasoning_effort,
            family.default_reasoning_effort
        );
        assert_eq!(updated.shell_type, family.shell_type);
    }
}
