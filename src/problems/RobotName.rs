use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};
use rand::{Rng, RngExt};

type FactoryId = usize;
type RobotName = String;


static FACTORY_COUNT: AtomicUsize  = AtomicUsize::new(0);
static FACTORIES: LazyLock<Mutex<HashMap<FactoryId, HashSet<RobotName>>>> = LazyLock::new(
    || Mutex::new(HashMap::new())
);


/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    id: FactoryId,
}

pub struct Robot {
    name: String,
    factory_id: FactoryId,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            id: FACTORY_COUNT.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, _rng: &mut R) -> Robot {
        Robot {
            factory_id: self.id,
            name: generate_name(self.id, _rng)
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset<R: Rng>(&mut self, _rng: &mut R) {
        let name = generate_name(self.factory_id, _rng);
        reset_name(self.factory_id, self.name.to_string());
        self.name = name;
    }
}


fn generate_name<R: Rng>(factory_id: FactoryId, _rng: &mut R) -> String {
    let mut factories = FACTORIES.lock().unwrap();
    let names = factories.entry(factory_id).or_default();

    loop {
        let letters: String = (0..2)
            .map(|_| _rng.random_range('A'..='Z'))
            .collect();

        let digits: String = _rng.random_range(100..1000).to_string();

        let name = format!("{letters}{digits}");

        if names.insert(name.clone()) {
            return name
        }
    }
}

fn reset_name(factory_id: FactoryId, name: String) {
    let mut factories = FACTORIES.lock().unwrap();
    let names = factories.entry(factory_id).or_default();

    names.remove(&name);
}