use schemars::JsonSchema;
/*
 * Jobs API 2.1
 *
 * The Jobs API allows you to create, edit, and delete jobs. You should never hard code secrets or store them in plain text. Use the [Secrets API](https://docs.databricks.com/dev-tools/api/latest/secrets.html) to manage secrets in the [Databricks CLI](https://docs.databricks.com/dev-tools/cli/index.html). Use the [Secrets utility](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-secrets) to reference secrets in notebooks and jobs.
 *
 * The version of the OpenAPI document: 2.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClusterAttributes {
    #[serde(rename = "data_security_mode", skip_serializing_if = "Option::is_none")]
    pub data_security_mode: Option<DataSecurityMode>,
    /// Cluster name requested by the user. This doesn’t have to be unique. If not specified at creation, the cluster name is an empty string.
    #[serde(rename = "cluster_name", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// The runtime version of the cluster, for example “5.0.x-scala2.11”. You can retrieve a list of available runtime versions by using the [Runtime versions](https://docs.databricks.com/dev-tools/api/latest/clusters.html#runtime-versions) API call.
    #[serde(rename = "spark_version", skip_serializing_if = "Option::is_none")]
    pub spark_version: Option<String>,
    /// An arbitrary object where the object key is a configuration propery name and the value is a configuration property value.
    #[serde(rename = "spark_conf", skip_serializing_if = "Option::is_none")]
    pub spark_conf: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "aws_attributes", skip_serializing_if = "Option::is_none")]
    pub aws_attributes: Option<Box<crate::models::AwsAttributes>>,
    /// This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads A list of available node types can be retrieved by using the [List node types](https://docs.databricks.com/dev-tools/api/latest/clusters.html#list-node-types) API call.
    #[serde(rename = "node_type_id", skip_serializing_if = "Option::is_none")]
    pub node_type_id: Option<String>,
    /// The node type of the Spark driver. This field is optional; if unset, the driver node type is set as the same value as `node_type_id` defined above.
    #[serde(
        rename = "driver_node_type_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub driver_node_type_id: Option<String>,
    /// SSH public key contents that is added to each Spark node in this cluster. The corresponding private keys can be used to login with the user name `ubuntu` on port `2200`. Up to 10 keys can be specified.
    #[serde(rename = "ssh_public_keys", skip_serializing_if = "Option::is_none")]
    pub ssh_public_keys: Option<Vec<String>>,
    /// An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions>
    #[serde(rename = "custom_tags", skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "cluster_log_conf", skip_serializing_if = "Option::is_none")]
    pub cluster_log_conf: Option<Box<crate::models::ClusterLogConf>>,
    /// The configuration for storing init scripts. Any number of destinations can be specified. The scripts are executed sequentially in the order provided. If `cluster_log_conf` is specified, init script logs are sent to `<destination>/<cluster-ID>/init_scripts`.
    #[serde(rename = "init_scripts", skip_serializing_if = "Option::is_none")]
    pub init_scripts: Option<Vec<crate::models::InitScriptInfo>>,
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<Box<crate::models::DockerImage>>,
    /// An arbitrary object where the object key is an environment variable name and the value is an environment variable value.
    #[serde(rename = "spark_env_vars", skip_serializing_if = "Option::is_none")]
    pub spark_env_vars: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Automatically terminates the cluster after it is inactive for this time in minutes. If not set, this cluster is not be automatically terminated. If specified, the threshold must be between 10 and 10000 minutes. You can also set this value to 0 to explicitly disable automatic termination.
    #[serde(
        rename = "autotermination_minutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub autotermination_minutes: Option<i32>,
    /// Autoscaling Local Storage: when enabled, this cluster dynamically acquires additional disk space when its Spark workers are running low on disk space. This feature requires specific AWS permissions to function correctly. Refer to [Autoscaling local storage](https://docs.databricks.com/clusters/configure.html#autoscaling-local-storage) for details.
    #[serde(
        rename = "enable_elastic_disk",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_elastic_disk: Option<bool>,
    /// The optional ID of the instance pool to which the cluster belongs. Refer to [Pools](https://docs.databricks.com/clusters/instance-pools/index.html) for details.
    #[serde(rename = "instance_pool_id", skip_serializing_if = "Option::is_none")]
    pub instance_pool_id: Option<String>,
    #[serde(rename = "cluster_source", skip_serializing_if = "Option::is_none")]
    pub cluster_source: Option<crate::models::ClusterSource>,
    /// A [cluster policy](https://docs.databricks.com/dev-tools/api/latest/policies.html) ID.
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}

impl ClusterAttributes {
    pub fn new() -> ClusterAttributes {
        ClusterAttributes {
            data_security_mode: None,
            cluster_name: None,
            spark_version: None,
            spark_conf: None,
            aws_attributes: None,
            node_type_id: None,
            driver_node_type_id: None,
            ssh_public_keys: None,
            custom_tags: None,
            cluster_log_conf: None,
            init_scripts: None,
            docker_image: None,
            spark_env_vars: None,
            autotermination_minutes: None,
            enable_elastic_disk: None,
            instance_pool_id: None,
            cluster_source: None,
            policy_id: None,
        }
    }
}

///
#[derive(
    JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum DataSecurityMode {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "LEGACY_TABLE_ACL")]
    LegacyTableAcl,
    #[serde(rename = "LEGACY_SINGLE_USER_STANDARD")]
    LegacySingleUserStandard,
}

impl Default for DataSecurityMode {
    fn default() -> DataSecurityMode {
        Self::None
    }
}
