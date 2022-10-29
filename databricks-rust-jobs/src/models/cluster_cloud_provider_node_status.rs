/*
 * Jobs API 2.1
 *
 * The Jobs API allows you to create, edit, and delete jobs. You should never hard code secrets or store them in plain text. Use the [Secrets API](https://docs.databricks.com/dev-tools/api/latest/secrets.html) to manage secrets in the [Databricks CLI](https://docs.databricks.com/dev-tools/cli/index.html). Use the [Secrets utility](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-secrets) to reference secrets in notebooks and jobs.
 *
 * The version of the OpenAPI document: 2.1
 *
 * Generated by: https://openapi-generator.tech
 */
use schemars::JsonSchema;

/// ClusterCloudProviderNodeStatus : * NotEnabledOnSubscription: Node type not available for subscription. * NotAvailableInRegion: Node type not available in region.

/// * NotEnabledOnSubscription: Node type not available for subscription. * NotAvailableInRegion: Node type not available in region.
#[derive(
    JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ClusterCloudProviderNodeStatus {
    #[serde(rename = "NotEnabledOnSubscription")]
    NotEnabledOnSubscription,
    #[serde(rename = "NotAvailableInRegion")]
    NotAvailableInRegion,
}

impl ToString for ClusterCloudProviderNodeStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NotEnabledOnSubscription => String::from("NotEnabledOnSubscription"),
            Self::NotAvailableInRegion => String::from("NotAvailableInRegion"),
        }
    }
}

impl Default for ClusterCloudProviderNodeStatus {
    fn default() -> ClusterCloudProviderNodeStatus {
        Self::NotEnabledOnSubscription
    }
}
