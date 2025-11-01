use crate::config::load::Parameters;
use custom_logger as log;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraResponse {
    pub expand: String,
    pub id: String,
    #[serde(rename = "self")]
    pub self_field: String,
    pub key: String,
    pub fields: Fields,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub priority: Priority,
    pub labels: Vec<String>,
    pub timeestimate: Value,
    pub versions: Vec<Value>,
    pub issuelinks: Vec<IssueLink>,
    pub assignee: Value,
    pub status: Status,
    pub components: Vec<Value>,
    pub creator: Creator,
    pub subtasks: Vec<Value>,
    pub aggregateprogress: Aggregateprogress,
    pub progress: Progress,
    pub issuetype: Issuetype,
    pub timespent: Value,
    pub project: Project,
    pub aggregatetimespent: Value,
    pub resolutiondate: Value,
    pub created: String,
    pub updated: String,
    pub timeoriginalestimate: Value,
    pub description: String,
    pub timetracking: Timetracking,
    pub attachment: Vec<Value>,
    pub summary: String,
    pub environment: Value,
    pub duedate: Value,
    pub comment: Comment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    #[serde(rename = "self")]
    pub self_field: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "self")]
    pub self_field: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
    pub status_category: StatusCategory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: i64,
    pub key: String,
    pub color_name: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    #[serde(rename = "self")]
    pub self_field: String,
    pub name: String,
    pub key: String,
    pub email_address: String,
    pub display_name: String,
    pub active: bool,
    pub time_zone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregateprogress {
    pub progress: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub progress: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Votes {
    #[serde(rename = "self")]
    pub self_field: String,
    pub votes: i64,
    pub has_voted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Worklog {
    pub start_at: i64,
    pub max_results: i64,
    pub total: i64,
    pub worklogs: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuetype {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub subtask: bool,
    pub avatar_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub project_type_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Watches {
    #[serde(rename = "self")]
    pub self_field: String,
    pub watch_count: i64,
    pub is_watching: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timetracking {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub comments: Vec<CommentData>,
    pub max_results: i64,
    pub total: i64,
    pub start_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentData {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub author: Author,
    pub body: String,
    pub created: String,
    pub updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    #[serde(rename = "self")]
    pub self_field: String,
    pub name: String,
    pub key: String,
    pub email_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLink {
    pub id: String,
    pub outward_issue: Option<OutwardIssue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutwardIssue {
    pub key: String,
}

pub trait ServiceInterface {
    async fn execute(
        params: Parameters,
        issue: String,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct Service {}

impl ServiceInterface for Service {
    async fn execute(
        params: Parameters,
        issue: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        log::info!("mode: execute");
        let api_key = fs::read_to_string(params.api_key_path)?;
        let url = format!("{}{}", params.base_url, issue);
        log::debug!("[execute] url {}", url);
        let client = reqwest::Client::new();
        let res = client.get(url).bearer_auth(api_key.trim()).send().await;
        match res {
            Ok(data) => {
                // epics
                let data_result = data.bytes().await?;
                let jira: JiraResponse = serde_json::from_slice(&data_result)?;
                save_data(jira.clone(), "EPIC".to_string())?;
                // linked user stories
                for link in jira.fields.issuelinks.iter() {
                    // this will fail if there is no issuelink
                    let story_url = format!(
                        "{}{}",
                        params.base_url,
                        link.outward_issue.as_ref().unwrap().key
                    );
                    let res_story = client
                        .get(story_url)
                        .bearer_auth(api_key.trim())
                        .send()
                        .await;
                    match res_story {
                        Ok(story) => {
                            let data_story = story.bytes().await?;
                            let jira: JiraResponse = serde_json::from_slice(&data_story)?;
                            save_data(jira.clone(), "STORY".to_string())?;
                        }
                        Err(err) => {
                            return Err(Box::from(err.to_string()));
                        }
                    }
                }
                Ok("exit => 0".to_string())
            }
            Err(err) => {
                return Err(Box::from(err.to_string()));
            }
        }
    }
}

fn save_data(jira: JiraResponse, jira_type: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("docs/weekly-status-report.md")?;

    let mut data = String::new();
    match jira_type.as_str() {
        "EPIC" => {
            data.push_str(&format!(
                "## [EPIC] [{}] {}",
                jira.key,
                jira.fields.summary.trim(),
            ));
            data.push_str(&format!(
                "\n\n### Status : {}",
                jira.fields.status.status_category.name.trim()
            ));
            data.push_str("\n\n### Description\n\n");
            let lines = jira.fields.description.split("\n").collect::<Vec<&str>>();
            for line in lines.iter() {
                data.push_str(&format!(
                    "\t{}\n",
                    line.replace("\r", "").replace("* ", "- ").trim()
                ));
            }
            data.push_str("\n\n### Stories\n\n");
        }
        "STORY" => {
            if jira
                .fields
                .status
                .status_category
                .name
                .contains("In Progress")
            {
                data.push_str(&format!(
                    "\n**[{}] {}**\n",
                    jira.key,
                    jira.fields.summary.trim(),
                ));
                data.push_str(&format!(
                    "\n- **Status : {}**\n",
                    jira.fields.status.status_category.name.trim()
                ));
                data.push_str("\n- **Description**\n\n");
                let lines = jira.fields.description.split("\n").collect::<Vec<&str>>();
                for line in lines.iter() {
                    data.push_str(&format!(
                        "\t\t{}\n",
                        line.replace("\r", "").replace("* ", "- ").trim()
                    ));
                }
                data.push_str("\n- **Comments**\n");
                for comment in jira.fields.comment.comments {
                    data.push_str(&format!(
                        "\n\t- {} {} \n{}\n",
                        comment.author.name,
                        comment.created,
                        comment
                            .body
                            .trim()
                            .replace("{code:java}", "```bash")
                            .replace("{code}", "```")
                    ));
                }
            }
        }
        _ => {}
    }
    file.write_all(data.as_bytes())?;
    Ok(())
}
