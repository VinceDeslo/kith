#![allow(unused)]
use std::{collections::HashMap, fmt::format, io::{BufRead, BufReader}, iter::Map, process::{Command, Stdio}};
use tracing::{event, Level};
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Tsh {
    pub databases: Vec<Database>,
}

pub struct ConnectionArgs {
    pub instance: String,
    pub db_user: String,
    pub db_name: String,
}

impl Tsh {
    pub fn new() -> Tsh {
        Tsh { databases: vec![] }
    }

    pub fn login(&self, proxy_name: &str, cluster: &str) {
        event!(Level::DEBUG, "logging into teleport");

        let proxy = format!("--proxy={}", proxy_name);

        let teleport_cmd = Command::new("tsh")
            .args(["login", &proxy, cluster])
            .stdout(Stdio::piped())
            .output();

        match teleport_cmd {
            Ok(teleport_cmd) => {
                if let Ok(stdout) = String::from_utf8(teleport_cmd.stdout) {
                    event!(Level::DEBUG, "teleport login output");
                    event!(Level::DEBUG, stdout);
                } else {
                    event!(Level::ERROR, "failed to parse stdout from teleport login");
                }
            }
            Err(err) => {
                event!(Level::ERROR, "failed to get ouput from teleport login: {}", err);
            }
        }
    }

    // Database connection currently spawns a new terminal via AppleScript.
    // This is done to avoid the complexity of spawning an interactive child of psql
    // in the current terminal session. Also ensures that we don't have any zombie processes.
    // This should eventually be refactored into an OS agnostic approach.
    pub fn connect(&self, args: ConnectionArgs) {
        event!(Level::INFO, "Connecting...");

        let connection_command = format!(
            "tsh db connect --db-user={} --db-name={} {}",
            args.db_user,
            args.db_name,
            &args.instance,
        );

        let script = format!(
            r#"
            tell application "Terminal"
                activate
                do script "{}"
            end tell
            "#,
            connection_command,
        );

        event!(Level::INFO, script);

        let status = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .status()
            .expect("failed to execute AppleScript");

        if !status.success() {
            event!(Level::ERROR, "teleport database connection failed with status: {}", status);
        }
    }

    fn disconnect(&self) {
        Command::new("tsh")
            .args(["db", "logout"])
            .stdout(Stdio::piped())
            .output()
            .expect("failed to disconnect from the teleport database");
    }

    pub fn read_databases(&mut self, database_name: &str) {
        event!(Level::DEBUG, "reading teleport databases");

        // Ensure we are disconnected from any instances
        self.disconnect();

        let search = format!("--search={}", database_name);
        let format = format!("--format={}", "json");
    
        let teleport_cmd = Command::new("tsh")
            .args(["db", "ls", &search, &format])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to list teleport databases");

        let out = teleport_cmd.stdout.expect("failed to open stdout");

        let mut reader = BufReader::new(out);
        let db_list: Vec<Database> = serde_json::
            from_reader(&mut reader)
            .expect("failed to deserialize database list");

        for db in db_list {
            let db_name = db.metadata.name.clone();
            event!(Level::DEBUG, "database: {}", db_name);
            self.databases.push(db);
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub metadata: Metadata,
    pub spec: Spec,
    pub users: Users,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub revision: String,
    pub labels: HashMap<String, String>
}

#[derive(Debug, Clone, Deserialize)]
pub struct Users {
    pub allowed: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    pub protocol: String,
    pub uri: String,
    pub aws: AwsSpec,
    pub gcp: GcpSpec,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AwsSpec {
    pub region: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GcpSpec {
    pub project_id: Option<String>,
    pub instance_id: Option<String>,
}

enum Fields {
    Name,
    Description,
    Protocol,
    Uri,
    AwsRegion,
    GcpProject,
    GcpInstance,
    AllowedUsers,
    Labels,
}

impl Fields {
    fn to_string(&self) -> &str {
        match self {
            Fields::Name => "Name",
            Fields::Description => "Description",
            Fields::Protocol => "Protocol",
            Fields::Uri => "URI",
            Fields::AwsRegion => "AWS Region",
            Fields::GcpProject => "GCP Project",
            Fields::GcpInstance => "GCP Instance",
            Fields::AllowedUsers => "Allowed Users",
            Fields::Labels => "Labels",
        }
    }
}

impl Database {
    pub fn connect(&self, db_name: String, db_user: String) {
        let db_user_arg = format!("--db-user={}", db_user);
        let db_name_arg = format!("--db-name={}", db_name);

        let mut cmd = Command::new("tsh")
            .args([
                "db",
                "connect",
                db_user_arg.as_str(),
                db_name_arg.as_str(),
                self.metadata.name.as_str(),
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn database connection command");

        let status = cmd.wait().expect("failed to wait on connection cmd");
        if status.success() {
            event!(Level::INFO, "tsh connection command succeeded");
        } else {
            event!(Level::ERROR, "tsh connection command failed");
        }
    }

    pub fn format_details(&self) -> String {
        let mut details = String::new();

        let name = format!(
            "{}: {}\n", 
            Fields::Name.to_string(), 
            self.metadata.name
        );
        details.push_str(name.as_str());

        let description = format!(
            "{}: {}\n", 
            Fields::Description.to_string(), 
            self.metadata.name
        );
        details.push_str(description.as_str());

        let protocol = format!(
            "{}: {}\n",
            Fields::Protocol.to_string(),
            self.spec.protocol
        );
        details.push_str(protocol.as_str());

        let uri = format!(
            "{}: {}\n", 
            Fields::Uri.to_string(), 
            self.spec.uri
        );
        details.push_str(uri.as_str());

        let is_aws = self.spec.aws.region.is_some();
        if is_aws {
            let aws_region = format!(
                "{}: {}\n",
                Fields::AwsRegion.to_string(),
                self.spec.aws.region
                    .clone()
                    .unwrap_or("unknown".to_string()),
            );
            details.push_str(aws_region.as_str());
        }
        let is_gcp = self.spec.gcp.project_id.is_some();
        if is_gcp {
            let gcp_project = format!(
                "{}: {}\n",
                Fields::GcpProject.to_string(),
                self.spec.gcp.project_id
                    .clone()
                    .unwrap_or("unknown".to_string()),
            );
            details.push_str(gcp_project.as_str());

            let gcp_instance = format!(
                "{}: {}\n",
                Fields::GcpInstance.to_string(),
                self.spec.gcp.instance_id
                    .clone()
                    .unwrap_or("unknown".to_string()),
            );
            details.push_str(gcp_instance.as_str());
        }

        let allowed_users_list = self.users.allowed
            .iter()
            .fold(String::new(), |mut accumulator, element| {
                accumulator.push_str(
                    format!("  - {}\n", element).as_str()
                );
                accumulator
            });

        let allowed_users = format!(
            "{}:\n{}",
            Fields::AllowedUsers.to_string(),
            allowed_users_list
        );
        details.push_str(allowed_users.as_str());

        let labels_list = self.metadata.labels
            .iter()
            .fold(String::new(), |mut accumulator, element| {
                accumulator.push_str(
                    format!("  - {}: {}\n", element.0, element.1).as_str()
                );
                accumulator
            });

        let labels  = format!(
            "{}:\n{}",
            Fields::Labels.to_string(),
            labels_list
        );
        details.push_str(labels.as_str());

        details
    }
}
