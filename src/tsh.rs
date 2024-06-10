#![allow(dead_code)]
#![allow(unused)]
use std::{collections::HashMap, io::{BufRead, BufReader}, process::{Command, Stdio}};
use log::{error, info, debug};

enum Lines {
    Separators,
    FirstEntry,
}

impl Lines {
    fn to_usize(&self) -> usize {
        match self {
            // Index 0 would be the column names, but these are hard coded instead.
            Lines::Separators => 1,
            Lines::FirstEntry => 2,
        }
    }
}

// Column definitions can be found in the Telepor repo
// https://github.com/gravitational/teleport/blob/abc6511f4016a4695062d53076b96ed1d05fec72/tool/tsh/common/db_print.go#L33
enum Columns {
    Name,
    Description,
    Protocol,
    DatabaseType,
    Uri,
    AllowedUsers,
    DatabaseRoles,
    Labels,
    Connect,
}

impl Columns {
    fn to_usize(&self) -> usize {
        match self {
            Columns::Name => 0,
            Columns::Description => 1,
            Columns::Protocol => 2,
            Columns::DatabaseType => 3,
            Columns::Uri => 4,
            Columns::AllowedUsers => 5,
            Columns::DatabaseRoles => 6,
            Columns::Labels => 7,
            Columns::Connect => 8,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Columns::Name => "Name",
            Columns::Description => "Description",
            Columns::Protocol => "Protocol",
            Columns::DatabaseType => "Database Type",
            Columns::Uri => "URI",
            Columns::AllowedUsers => "Allowed Users",
            Columns::DatabaseRoles => "Database Roles",
            Columns::Labels => "Labels",
            Columns::Connect => "Connect",
        }
    }
}

#[derive(Debug, Default)]
pub struct Tsh {
    lines: Vec<String>,
    column_widths: Vec<usize>,
    raw_entries: Vec<String>,
    entries: Vec<DatabaseEntry>,
}

#[derive(Debug, Default)]
struct DatabaseEntry {
    name: String, 
    description: String,
    protocol: String,
    database_type: String,
    uri: String,                                                                           
    allowed_users: Vec<String>,
    database_roles: String,
    labels: HashMap<String, String>,
    connect: String,
}

impl Tsh {
    pub fn new() -> Tsh {
        return Tsh {
            lines: vec![],
            column_widths: vec![],
            raw_entries: vec![],
            entries: vec![],
        }
    }

    pub fn login(&self, proxy_name: &str, cluster: &str) {
        info!("logging into teleport");

        let proxy = format!("--proxy={}", proxy_name);

        let teleport_cmd = Command::new("tsh")
            .args(["login", &proxy, &cluster])
            .stdout(Stdio::piped())
            .output();

        match teleport_cmd {
            Ok(teleport_cmd) => {
                if let Ok(stdout) = String::from_utf8(teleport_cmd.stdout) {
                    info!("teleport login output");
                    info!("{}", stdout)
                } else {
                    error!("failed to parse stdout from teleport login")
                }
            }
            Err(teleport_cmd) => {
                error!("failed to get ouput from teleport login");
            }
        }
    }

    pub fn read_databases(&mut self, database_name: &str) {
        info!("reading teleport databases");

        let search = format!("--search={}", database_name);
    
        let teleport_cmd = Command::new("tsh")
            .args(["db", "ls", "-v", &search])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to list teleport databases");

        let out = teleport_cmd.stdout.expect("failed to open stdout");
    
        let reader = BufReader::new(out);

        for line in reader.lines() {
            let line_val = line.expect("failed to read line from stdout");
            self.lines.push(line_val);
        }
        
        info!("parsing teleport databases");
        self.parse_separators();
        self.parse_raw_entries();
        self.parse_entries();
    }

    // column widths are parsed from the second line of Teleport 
    // command output. These can be used to know what columns may be empty
    fn parse_separators(&mut self) {
        info!("parsing separators");

        let column_separators = &self.lines[Lines::Separators.to_usize()];

        self.column_widths = column_separators
            .split(" ")
            .map(|separator| separator.len())
            .filter(|column_width| *column_width != 0)
            .collect();

        debug!("{:?}", self.column_widths);
    }

    fn parse_raw_entries(&mut self) {
        info!("parsing raw entries");

        let raw_entries = &self.lines[Lines::FirstEntry.to_usize()..];

        self.raw_entries = raw_entries
            .into_iter()
            .map(|raw_entry| raw_entry.to_string())
            .filter(|raw_entry| !raw_entry.is_empty())
            .collect();

        debug!("{:?}", self.raw_entries);
    }

    fn parse_entries(&mut self) {
        info!("parsing entries");

        let column_widths = self.column_widths.clone();

        self.entries = self.raw_entries
            .clone()
            .into_iter()
            .map(|raw_entry| {
                DatabaseEntry { 
                    name: parse_column(
                        Columns::Name,
                        &raw_entry,
                        &column_widths,
                    ),
                    description: parse_column(
                        Columns::Description,
                        &raw_entry,
                        &column_widths,
                    ),
                    protocol: parse_column(
                        Columns::Protocol,
                        &raw_entry,
                        &column_widths,
                    ),
                    database_type: parse_column(
                        Columns::DatabaseType,
                        &raw_entry,
                        &column_widths,
                    ),
                    uri: parse_column(
                        Columns::Uri,
                        &raw_entry,
                        &column_widths,
                    ),
                    allowed_users: parse_allowed_users(
                        Columns::AllowedUsers,
                        &raw_entry,
                        &column_widths,
                    ),
                    database_roles: parse_column(
                        Columns::DatabaseRoles,
                        &raw_entry,
                        &column_widths,
                    ),
                    labels: parse_labels(
                        Columns::Labels,
                        &raw_entry,
                        &column_widths,
                    ),
                    connect: parse_column(
                        Columns::Connect,
                        &raw_entry,
                        &column_widths,
                    ),
                }
            })
            .collect();

        debug!("{:?}", self.entries);
    }
}

fn get_column_bounds(column_index: usize, column_widths: Vec<usize>) -> (usize, usize) {
    let sum = column_widths
        .iter()
        .take(column_index)
        .map(|width| width + 1) // Accomodate the extra space between columns
        .sum();

    return (sum, sum + column_widths[column_index]);
}

fn parse_column(column: Columns, raw_entry: &String, column_widths: &Vec<usize>) -> String {
    info!("Parsing column: {}", column.to_string());

    let width = column_widths[column.to_usize()];

    let bounds = get_column_bounds(column.to_usize(), column_widths.to_vec());

    debug!("Column bounds: ({}, {})", bounds.0, bounds.1);

    let column_value = raw_entry
        .split_at(bounds.0).1
        .split_at(width).0;

    debug!("{}\n", column_value);

    return column_value.trim().to_string();
}

fn parse_allowed_users(column: Columns, raw_entry: &String, column_widths: &Vec<usize>) -> Vec<String> {
    let column_value = parse_column(column, raw_entry, column_widths);

    return column_value
        .replace("[", "")
        .replace("]", "")
        .split(" ")
        .map(|user| user.to_string())
        .collect();
}

fn parse_roles(column: Columns, raw_entry: &String, column_widths: &Vec<usize>) -> Vec<String> {
    let column_value = parse_column(column, raw_entry, column_widths);
    return vec![column_value];
}

fn parse_labels(column: Columns, raw_entry: &String, column_widths: &Vec<usize>) -> HashMap<String, String> {
    let column_value = parse_column(column, raw_entry, column_widths);

    let mut map: HashMap<String, String> = HashMap::new();

    column_value
        .split(",")
        .map(|label| label_to_key_value(label))
        .for_each(|parsed_label| {
            map.insert(parsed_label.0, parsed_label.1);
        });

    return map;
}

fn label_to_key_value(label: &str) -> (String, String) {
    let label_members: Vec<&str> = label.split("=").collect();
    let key = label_members[0].to_string();
    let value = label_members[1].to_string();
    (key, value)
}
