use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt;
use std::{error::Error, fs::File};

#[derive(Deserialize)]
pub struct Team {
    pub num: u32,
    pub region: String,
    pub school: String,
}

impl fmt::Debug for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "num: {}, region: {}, school: {}\n",
            self.num, self.region, self.school
        )
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        (&self.num, &self.region, &self.school) == (&other.num, &other.region, &other.school)
    }
}
impl Eq for Team {}
impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.num, &self.school).cmp(&(&other.num, &other.school))
    }
}
impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn read_teams(region: &str) -> Result<Vec<Team>, std::boxed::Box<dyn Error>> {
    let mut v: Vec<Team> = vec![];
    let file = File::open("Team-List-2025.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let row: Team = record.deserialize(None)?;
        if row.region == region {
            v.push(row);
        }
    }
    // Sort in reverse order because we add to GUI by popping from top
    v.sort_by(|a, b| b.cmp(a));
    Ok(v)
}
