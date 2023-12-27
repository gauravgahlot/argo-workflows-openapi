use serde::{Deserialize, Serialize};

/// WorkflowStep is a reference to a template to execute in a series of step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStep {
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Box<super::Arguments>>,

    #[serde(rename = "continueOn", skip_serializing_if = "Option::is_none")]
    pub continue_on: Option<Box<super::ContinueOn>>,

    /// Hooks holds the lifecycle hook which is invoked at lifecycle of step,
    /// irrespective of the success, failure, or error status of the primary step.
    #[serde(rename = "hooks", skip_serializing_if = "Option::is_none")]
    pub hooks: Option<::std::collections::HashMap<String, super::LifecycleHook>>,

    #[serde(rename = "inline", skip_serializing_if = "Option::is_none")]
    pub inline: Option<Box<super::Template>>,
    
    /// Name of the step.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// OnExit is a template reference which is invoked at the end of the
    /// template, irrespective of the success, failure, or error of the primary
    /// template. DEPRECATED: Use Hooks[exit]. Template instead.
    #[serde(rename = "onExit", skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<String>,

    /// Template is the name of the template to execute as the step.
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(rename = "templateRef", skip_serializing_if = "Option::is_none")]
    pub template_ref: Option<Box<super::TemplateRef>>,

    /// When is an expression in which the step should conditionally execute.
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,

    /// WithItems expands a step into multiple parallel steps
    /// from the items in the list.
    #[serde(rename = "withItems", skip_serializing_if = "Option::is_none")]
    pub with_items: Option<Vec<serde_json::Value>>,

    /// WithParam expands a step into multiple parallel steps from the value
    /// in the parameter, which is expected to be a JSON list.
    #[serde(rename = "withParam", skip_serializing_if = "Option::is_none")]
    pub with_param: Option<String>,

    #[serde(rename = "withSequence", skip_serializing_if = "Option::is_none")]
    pub with_sequence: Option<Box<super::Sequence>>,
}

impl WorkflowStep {
    pub fn new() -> WorkflowStep {
        WorkflowStep {
            arguments: None,
            continue_on: None,
            hooks: None,
            inline: None,
            name: None,
            on_exit: None,
            template: None,
            template_ref: None,
            when: None,
            with_items: None,
            with_param: None,
            with_sequence: None,
        }
    }
}
