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

#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SqlTaskAlert {
    /// The canonical identifier of the SQL alert.
    #[serde(rename = "alert_id")]
    pub alert_id: String,
}

impl SqlTaskAlert {
    pub fn new(alert_id: String) -> SqlTaskAlert {
        SqlTaskAlert { alert_id }
    }
}
