use std::fmt;

use chrono::{DateTime, Local};
use json::JsonValue;

// #[derive(Debug, Default)]
// pub struct Error {
//     pub detail: String,
// }
//
// #[derive(Debug)]
// pub struct CrateLinks {
//     pub owners: Option<String>,
//     pub reverse_dependencies: String,
//     pub version_downloads: String,
//     pub versions: Option<String>,
// }
//
// pub struct Crate {
//     pub created_at: String,
//     pub description: Option<String>,
//     pub documentation: Option<String>,
//     pub downloads: i32,
//     pub homepage: Option<String>,
//     pub id: String,
//     pub keywords: Option<Vec<String>>,
//     pub license: Option<String>,
//     pub links: CrateLinks,
//     pub max_version: String,
//     pub name: String,
//     pub repository: Option<String>,
//     pub updated_at: String,
//     pub versions: Option<Vec<u64>>,
// }
//
// #[derive(Debug)]
// pub struct Keyword {
//     pub crates_cnt: u64,
//     pub created_at: String,
//     pub id: String,
//     pub keyword: String,
// }
//
// #[derive(Debug)]
// pub struct VersionLinks {
//     pub authors: String,
//     pub dependencies: String,
//     pub version_downloads: String,
// }
//
// #[derive(Debug)]
// pub struct Version {
//     pub krate: String,
//     pub created_at: String,
//     pub dl_path: String,
//     pub downloads: i32,
//     pub features: HashMap<String, Vec<String>>,
//     pub id: i32,
//     pub links: VersionLinks,
//     pub num: String,
//     pub updated_at: String,
//     pub yanked: bool,
// }
//
// pub struct Reply {
//     pub errors: Error,
//     pub krate: Crate,
//     pub keywords: Vec<Keyword>,
//     pub versions: Vec<Version>,
// }

struct TimeStamp(Option<DateTime<Local>>);

impl<'a> From<&'a JsonValue> for TimeStamp {
    fn from(jv: &JsonValue) -> Self {
        let parse = |s: &str| s.parse::<DateTime<Local>>().ok();
        TimeStamp(jv.as_str().and_then(parse))
    }
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ts) = self.0 {
            write!(f, "{}", ts)
        } else {
            write!(f, "")
        }
    }
}

pub struct Crate {
    json: JsonValue,
}

impl Crate {
    pub fn new(json: JsonValue) -> Self {
        Crate { json: json }
    }

    pub fn print_repository(&self, verbose: bool) {
        if let JsonValue::String(ref repository) = self.json["repository"] {
            let fmt = if verbose {
                format!("{:<16}{}", "Repository:", repository)
            } else {
                repository.clone()
            };
            println!("{}", fmt);
        }
    }

    pub fn print_documentation(&self, verbose: bool) {
        if let JsonValue::String(ref documentation) = self.json["documentation"] {
            let fmt = if verbose {
                format!("{:<16}{}", "Documentation:", documentation)
            } else {
                documentation.clone()
            };
            println!("{}", fmt);
        }
    }

    pub fn print_downloads(&self, verbose: bool) {
        if let JsonValue::Number(downloads) = self.json["downloads"] {
            let fmt = if verbose {
                format!("{:<16}{}", "Downloads:", downloads)
            } else {
                format!("{}", downloads)
            };
            println!("{}", fmt);
        }
    }

    pub fn print_homepage(&self, verbose: bool) {
        if let JsonValue::String(ref homepage) = self.json["homepage"] {
            let fmt = if verbose {
                format!("{:<16}{}", "Homepage:", homepage)
            } else {
                homepage.clone()
            };
            println!("{}", fmt);
        }
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let empty = "";

        let name = self.json["name"].as_str().unwrap_or(empty);
        let max_version = self.json["max_version"].as_str().unwrap_or(empty);
        let downloads = self.json["downloads"].as_i32().unwrap_or(0);

        let created_at = TimeStamp::from(&self.json["created_at"]);
        let updated_at = TimeStamp::from(&self.json["updated_at"]);

        let description = self.json["description"].as_str().unwrap_or(empty);
        let documentation = self.json["documentation"].as_str().unwrap_or(empty);
        let homepage = self.json["homepage"].as_str().unwrap_or(empty);
        let repository = self.json["repository"].as_str().unwrap_or(empty);
        let license = self.json["license"].as_str().unwrap_or(empty);

        let keywords = self.json["keywords"]
            .members()
            .filter_map(|jv| jv.as_str())
            .collect::<Vec<_>>();

        if f.alternate() {
            write!(f,
                   "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                   format_args!("{:<16}{}", "Crate:", name),
                   format_args!("{:<16}{}", "Version:", max_version),
                   format_args!("{:<16}{}", "Description:", description),
                   format_args!("{:<16}{}", "Downloads:", downloads),
                   format_args!("{:<16}{}", "Homepage:", homepage),
                   format_args!("{:<16}{}", "Documentation:", documentation),
                   format_args!("{:<16}{}", "Repository:", repository),
                   format_args!("{:<16}{}", "License:", license),
                   format_args!("{:<16}{:?}", "Keywords:", keywords),
                   format_args!("{:<16}{}", "Created at:", created_at),
                   format_args!("{:<16}{}", "Updated at:", updated_at))
        } else {
            write!(f,
                   "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                   format_args!("{:<16}{}", "Crate:", name),
                   format_args!("{:<16}{}", "Version:", max_version),
                   format_args!("{:<16}{}", "Description:", description),
                   format_args!("{:<16}{}", "Downloads:", downloads),
                   format_args!("{:<16}{}", "Homepage:", homepage),
                   format_args!("{:<16}{}", "Documentation:", documentation),
                   format_args!("{:<16}{}", "Repository:", repository))
        }
    }
}
