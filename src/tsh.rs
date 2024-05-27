#![allow(dead_code)]
#![allow(unused)]
use std::{collections::HashMap, io::{BufRead, BufReader}, process::{Command, Stdio}};
use log::{error, info};

const COLUMNS_LINE: usize = 0;
const SEPARATORS_LINE: usize = 1;
const FIRST_ENTRY_LINE: usize = 2;

const NAME_COLUMN: usize = 0;
const DESCRIPTION_COLUMN: usize = 1;
const PROTOCOL_COLUMN: usize = 2;
const DATABASE_TYPE_COLUMN: usize = 3;
const URI_COLUMN: usize = 4;
const ALLOWED_USERS_COLUMN: usize = 5;
const DATABASE_COLUMN: usize = 6;
const ROLES_COLUMN: usize = 7;
const LABELS_COLUMN: usize = 8;
const CONNECT_COLUMN: usize = 9;

#[derive(Debug, Default)]
pub struct Tsh {
    lines: Vec<String>,
    columns: Vec<String>,
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
    database: String,
    roles: Vec<String>,
    labels: HashMap<String, String>,
    connect: String,
}

impl Tsh {
    pub fn new() -> Tsh {
        return Tsh {
            lines: vec![],
            columns: vec![],
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
        self.parse_columns();
        self.parse_separators();
        self.parse_raw_entries();
        self.parse_entries();
    }

    // column names are extracted from the first line of Teleport command output
    fn parse_columns(&mut self) {
        info!("parsing columns");

        let column_names = &self.lines[COLUMNS_LINE];
        
        self.columns = column_names
            .split(" ")
            .map(|name| name.to_string())
            .collect(); 
    }

    // column widths are parsed from the second line of Teleport 
    // command output. These can be used to know what columns may be empty
    fn parse_separators(&mut self) {
        info!("parsing separators");

        let column_separators = &self.lines[SEPARATORS_LINE];

        self.column_widths = column_separators
            .split(" ")
            .map(|separator| separator.len())
            .collect();
    }

    fn parse_raw_entries(&mut self) {
        info!("parsing raw entries");

        let raw_entries = &self.lines[FIRST_ENTRY_LINE..];

        self.raw_entries = raw_entries
            .into_iter()
            .map(|raw_entry| raw_entry.to_string())
            .collect();
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
                        NAME_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    description: parse_column(
                        DESCRIPTION_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    protocol: parse_column(
                        PROTOCOL_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    database_type: parse_column(
                        DATABASE_TYPE_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    uri: parse_column(
                        URI_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    allowed_users: parse_allowed_users(
                        ALLOWED_USERS_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    database: parse_column(
                        DATABASE_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    roles: parse_roles(
                        ROLES_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    labels: parse_labels(
                        LABELS_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                    connect: parse_column(
                        CONNECT_COLUMN,
                        raw_entry.clone(),
                        column_widths.clone(),
                    ),
                }
            })
            .collect();
    }
}

// TODO: Need to work on this algorithm
fn get_column_bounds(column_index: usize, column_widths: Vec<usize>) -> (usize, usize) {
    match column_index {
        0 => return (column_index, column_widths[column_index]),
        _ => {
            let last_index = column_index - 1;  
            let sum = column_widths[..last_index]
                .iter()
                .sum();
            return (sum, sum + column_widths[column_index]);
        }
    } 
}

fn parse_column(column_index: usize, raw_entry: String, column_widths: Vec<usize>) -> String {
    let bounds = get_column_bounds(column_index, column_widths);

    let column_value = raw_entry
        .split_at(bounds.0).1
        .split_at(bounds.1).0;

    info!("{}", column_value);

    return column_value.trim().to_string();
}

fn parse_allowed_users(column_index: usize, raw_entry: String, column_widths: Vec<usize>) -> Vec<String>{
    let column_value = parse_column(column_index, raw_entry, column_widths);
    return vec![column_value];
}

fn parse_roles(column_index: usize, raw_entry: String, column_widths: Vec<usize>) -> Vec<String> {
    let column_value = parse_column(column_index, raw_entry, column_widths);
    return vec![column_value];
}

fn parse_labels(column_index: usize, raw_entry: String, column_widths: Vec<usize>) -> HashMap<String, String> {
    let column_value = parse_column(column_index, raw_entry, column_widths);
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("key".to_string(), column_value.to_string());
    return map;
}
