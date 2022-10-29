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
pub struct JobsGet200Response {
    /// The canonical identifier for this job.
    #[serde(rename = "job_id", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
    /// The creator user name. This field won’t be included in the response if the user has been deleted.
    #[serde(rename = "creator_user_name", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    /// The user name that the job runs as. `run_as_user_name` is based on the current job settings, and is set to the creator of the job if job access control is disabled, or the `is_owner` permission if job access control is enabled.
    #[serde(rename = "run_as_user_name", skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::JobSettings>>,
    /// The time at which this job was created in epoch milliseconds (milliseconds since 1/1/1970 UTC).
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
}

impl JobsGet200Response {
    pub fn new() -> JobsGet200Response {
        JobsGet200Response {
            job_id: None,
            creator_user_name: None,
            run_as_user_name: None,
            settings: None,
            created_time: None,
        }
    }
}
