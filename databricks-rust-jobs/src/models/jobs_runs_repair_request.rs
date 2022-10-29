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
pub struct JobsRunsRepairRequest {
    /// The job run ID of the run to repair. The run must not be in progress.
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i64>,
    /// The task keys of the task runs to repair.
    #[serde(rename = "rerun_tasks", skip_serializing_if = "Option::is_none")]
    pub rerun_tasks: Option<Vec<String>>,
    /// The ID of the latest repair. This parameter is not required when repairing a run for the first time, but must be provided on subsequent requests to repair the same run.
    #[serde(rename = "latest_repair_id", skip_serializing_if = "Option::is_none")]
    pub latest_repair_id: Option<i64>,
    /// If true, repair all failed tasks. Only one of rerun_tasks or rerun_all_failed_tasks can be used.
    #[serde(
        rename = "rerun_all_failed_tasks",
        skip_serializing_if = "Option::is_none"
    )]
    pub rerun_all_failed_tasks: Option<bool>,
    #[serde(rename = "jar_params", skip_serializing_if = "Option::is_none")]
    pub jar_params: Option<Vec<String>>,
    /// A map from keys to values for jobs with notebook task, for example `\"notebook_params\": {\"name\": \"john doe\", \"age\": \"35\"}`. The map is passed to the notebook and is accessible through the [dbutils.widgets.get](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-widgets) function.  If not specified upon `run-now`, the triggered run uses the job’s base parameters.  notebook_params cannot be specified in conjunction with jar_params.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  The JSON representation of this field (for example `{\"notebook_params\":{\"name\":\"john doe\",\"age\":\"35\"}}`) cannot exceed 10,000 bytes.
    #[serde(rename = "notebook_params", skip_serializing_if = "Option::is_none")]
    pub notebook_params: Option<Vec<String>>,
    /// A list of parameters for jobs with Python tasks, for example `\"python_params\": [\"john doe\", \"35\"]`. The parameters are passed to Python file as command-line parameters. If specified upon `run-now`, it would overwrite the parameters specified in job setting. The JSON representation of this field (for example `{\"python_params\":[\"john doe\",\"35\"]}`) cannot exceed 10,000 bytes.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  Important  These parameters accept only Latin characters (ASCII character set). Using non-ASCII characters returns an error. Examples of invalid, non-ASCII characters are Chinese, Japanese kanjis, and emojis.
    #[serde(rename = "python_params", skip_serializing_if = "Option::is_none")]
    pub python_params: Option<Vec<String>>,
    /// A list of parameters for jobs with spark submit task, for example `\"spark_submit_params\": [\"--class\", \"org.apache.spark.examples.SparkPi\"]`. The parameters are passed to spark-submit script as command-line parameters. If specified upon `run-now`, it would overwrite the parameters specified in job setting. The JSON representation of this field (for example `{\"python_params\":[\"john doe\",\"35\"]}`) cannot exceed 10,000 bytes.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  Important  These parameters accept only Latin characters (ASCII character set). Using non-ASCII characters returns an error. Examples of invalid, non-ASCII characters are Chinese, Japanese kanjis, and emojis.
    #[serde(
        rename = "spark_submit_params",
        skip_serializing_if = "Option::is_none"
    )]
    pub spark_submit_params: Option<Vec<String>>,
    /// A map from keys to values for jobs with Python wheel task, for example `\"python_named_params\": {\"name\": \"task\", \"data\": \"dbfs:/path/to/data.json\"}`.
    #[serde(
        rename = "python_named_params",
        skip_serializing_if = "Option::is_none"
    )]
    pub python_named_params: Option<serde_json::Value>,
    #[serde(rename = "pipeline_params", skip_serializing_if = "Option::is_none")]
    pub pipeline_params: Option<Box<crate::models::RunParametersPipelineParams>>,
    /// A map from keys to values for SQL tasks, for example `\"sql_params\": {\"name\": \"john doe\", \"age\": \"35\"}`. The SQL alert task does not support custom parameters.
    #[serde(rename = "sql_params", skip_serializing_if = "Option::is_none")]
    pub sql_params: Option<serde_json::Value>,
    /// An array of commands to execute for jobs with the dbt task, for example `\"dbt_commands\": [\"dbt deps\", \"dbt seed\", \"dbt run\"]`
    #[serde(rename = "dbt_commands", skip_serializing_if = "Option::is_none")]
    pub dbt_commands: Option<Vec<String>>,
}

impl JobsRunsRepairRequest {
    pub fn new() -> JobsRunsRepairRequest {
        JobsRunsRepairRequest {
            run_id: None,
            rerun_tasks: None,
            latest_repair_id: None,
            rerun_all_failed_tasks: None,
            jar_params: None,
            notebook_params: None,
            python_params: None,
            spark_submit_params: None,
            python_named_params: None,
            pipeline_params: None,
            sql_params: None,
            dbt_commands: None,
        }
    }
}
