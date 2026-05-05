use std::collections::HashMap;

macro_rules! table_row {
    ($name:expr, $mp:expr, $w:expr, $d:expr, $l:expr, $p:expr) => {
        format!("{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", $name, $mp, $w, $d, $l, $p)
    };
}

#[derive(Default)]
struct Team {
    name: String,
    matches: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32
}

impl Team {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn win(&mut self) {
        self.wins += 1;
        self.matches += 1;
        self.points += 3;
    }

    fn draw(&mut self) {
        self.draws += 1;
        self.matches += 1;
        self.points += 1;
    }

    fn loss(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }

    fn add_match(&mut self, mr: &MathResult) {
        match mr {
            MathResult::Win => self.win(),
            MathResult::Draw => self.draw(),
            MathResult::Loss => self.loss(),
        }
    }
}

impl From<&Team> for String {
    fn from(team: &Team) -> Self {
        table_row!(
            team.name,
            team.matches,
            team.wins,
            team.draws,
            team.losses,
            team.points
        )
    }
}

enum MathResult {
    Win,
    Loss,
    Draw
}

impl From<&str> for MathResult {
    fn from(res: &str) -> Self {
        match res {
            "win" => MathResult::Win,
            "loss" => MathResult::Loss,
            "draw" => MathResult::Draw,
            _ => panic!(),
        }
    }
}

impl MathResult {
    fn invert(&self) -> MathResult {
        match self {
            MathResult::Win => MathResult::Loss,
            MathResult::Loss => MathResult::Win,
            MathResult::Draw => MathResult::Draw
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table: HashMap<String, Team> = HashMap::new();

    match_results.lines().for_each(|match_row| {
        let parts = match_row.split(";").collect::<Vec<&str>>();
        let home = parts[0];
        let guest = parts[1];

        // MathResult::from(parts[2]);
        let result = parts[2].into();


        table.entry(home.into()).or_insert(Team::new(home.into()))
            .add_match(&result);

        table.entry(guest.into()).or_insert(Team::new(guest.into()))
            .add_match(&result.invert());
    });

    let mut sorted_by_scores: Vec<&Team> = table.values().collect();
    sorted_by_scores.sort_by(|a, b| b.points.cmp(&a.points).then(a.name.cmp(&b.name)));

    vec![table_row!("Team", "MP", "W", "D", "L", "P")]
        // Vec<String> → Iterator<Item = String>
        .into_iter()
        .chain(sorted_by_scores.into_iter().map(|t| t.into()))
        .collect::<Vec<String>>()
        .join("\n")
}