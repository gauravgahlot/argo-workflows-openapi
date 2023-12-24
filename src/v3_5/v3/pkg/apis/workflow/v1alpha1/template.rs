use serde::{Deserialize, Serialize};
use k8s_openapi::api::core;

/// Template is a reusable and composable unit of execution in a workflow.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "activeDeadlineSeconds", skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<String>,

    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Box<core::v1::Affinity>>,
    
    #[serde(rename = "archiveLocation", skip_serializing_if = "Option::is_none")]
    pub archive_location: Option<Box<super::ArtifactLocation>>,

    /// AutomountServiceAccountToken indicates whether a service account 
    /// token should be automatically mounted in pods. ServiceAccountName 
    /// of ExecutorConfig must be specified if this value is false.
    #[serde(rename = "automountServiceAccountToken", skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,

    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<Box<core::v1::Container>>,

    #[serde(rename = "containerSet", skip_serializing_if = "Option::is_none")]
    pub container_set: Option<Box<super::ContainerSetTemplate>>,

    /// Daemon will allow a workflow to proceed to the next step so 
    /// long as the container reaches readiness.
    #[serde(rename = "daemon", skip_serializing_if = "Option::is_none")]
    pub daemon: Option<bool>,

    #[serde(rename = "dag", skip_serializing_if = "Option::is_none")]
    pub dag: Option<Box<super::DAGTemplate>>,

    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<super::Data>>,

    #[serde(rename = "executor", skip_serializing_if = "Option::is_none")]
    pub executor: Option<Box<super::ExecutorConfig>>,

    /// FailFast, if specified, will fail this template if any of its child
    /// pods has failed. This is useful for when this template is expanded
    /// with `withItems`, etc.
    #[serde(rename = "failFast", skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,

    /// HostAliases is an optional list of hosts and IPs that will be injected
    /// into the pod spec.
    #[serde(rename = "hostAliases", skip_serializing_if = "Option::is_none")]
    pub host_aliases: Option<Vec<core::v1::HostAlias>>,

    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<super::HTTP>>,

    /// InitContainers is a list of containers which run before the main container.
    #[serde(rename = "initContainers", skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<super::UserContainer>>,

    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Box<super::Inputs>>,

    #[serde(rename = "memoize", skip_serializing_if = "Option::is_none")]
    pub memoize: Option<Box<super::Memoize>>,

    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<super::Metadata>>,

    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Box<super::Metrics>>,

    /// Name is the name of the template
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// NodeSelector is a selector to schedule this step of the workflow to be run
    /// on the selected node(s). Overrides the selector set at the workflow level.
    #[serde(rename = "nodeSelector", skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<::std::collections::HashMap<String, String>>,

    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Box<super::Outputs>>,
    
    /// Parallelism limits the max total parallel pods that can execute at the same time
    /// within the boundaries of this template invocation. If additional steps/dag templates 
    /// are invoked, the pods created by those templates will not be counted towards this total.
    #[serde(rename = "parallelism", skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,

    /// Plugin is an Object with exactly one key
    #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
    pub plugin: Option<serde_json::Value>,
    
    /// PodSpecPatch holds strategic merge patch to apply against the pod spec.Allows 
    /// parameterization of container fields which are not strings (e.g. resource limits).
    #[serde(rename = "podSpecPatch", skip_serializing_if = "Option::is_none")]
    pub pod_spec_patch: Option<String>,

    /// Priority to apply to workflow pods.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    
    /// PriorityClassName to apply to workflow pods.
    #[serde(rename = "priorityClassName", skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<String>,
    
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<super::ResourceTemplate>>,
    
    #[serde(rename = "retryStrategy", skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<Box<super::RetryStrategy>>,
    
    /// If specified, the pod will be dispatched by specified scheduler.
    /// Or it will be dispatched by workflow scope scheduler if specified.
    /// If neither specified, the pod will be dispatched by default scheduler.
    #[serde(rename = "schedulerName", skip_serializing_if = "Option::is_none")]
    pub scheduler_name: Option<String>,

    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<Box<super::ScriptTemplate>>,
    
    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    pub security_context: Option<Box<core::v1::PodSecurityContext>>,

    /// ServiceAccountName to apply to workflow pods
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
    
    /// Sidecars is a list of containers which run alongside the main container.
    /// Sidecars are automatically killed when the main container completes
    #[serde(rename = "sidecars", skip_serializing_if = "Option::is_none")]
    pub sidecars: Option<Vec<super::UserContainer>>,

    /// Steps define a series of sequential/parallel workflow steps
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Vec<super::WorkflowStep>>>,

    #[serde(rename = "suspend", skip_serializing_if = "Option::is_none")]
    pub suspend: Option<Box<super::SuspendTemplate>>,

    #[serde(rename = "synchronization", skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<Box<super::Synchronization>>,
    
    /// Timeout allows to set the total node execution timeout duration counting from 
    /// the node's start time. This duration also includes time in which the node spends
    /// in Pending state. This duration may not be applied to Step or DAG templates.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,

    /// Tolerations to apply to workflow pods.
    #[serde(rename = "tolerations", skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<core::v1::Toleration>>,

    /// Volumes is a list of volumes that can be mounted by containers in a template.
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<core::v1::Volume>>,
}

impl Template {
    pub fn new() -> Template {
        Template {
            active_deadline_seconds: None,
            affinity: None,
            archive_location: None,
            automount_service_account_token: None,
            container: None,
            container_set: None,
            daemon: None,
            dag: None,
            data: None,
            executor: None,
            fail_fast: None,
            host_aliases: None,
            http: None,
            init_containers: None,
            inputs: None,
            memoize: None,
            metadata: None,
            metrics: None,
            name: None,
            node_selector: None,
            outputs: None,
            parallelism: None,
            plugin: None,
            pod_spec_patch: None,
            priority: None,
            priority_class_name: None,
            resource: None,
            retry_strategy: None,
            scheduler_name: None,
            script: None,
            security_context: None,
            service_account_name: None,
            sidecars: None,
            steps: None,
            suspend: None,
            synchronization: None,
            timeout: None,
            tolerations: None,
            volumes: None,
        }
    }
}
