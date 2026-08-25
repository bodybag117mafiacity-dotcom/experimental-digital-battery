use crate::power_unit::PowerUnit;

#[derive(Debug)]
pub struct DigitalBattery {
    pub capacity: f64,
    pub level: f64,
    pub overload: bool,
    pub health: f64,
    pub workers: Vec<PowerUnit>,   // plusieurs workers
}

impl DigitalBattery {
    pub fn new(capacity: f64) -> Self {
        Self {
            capacity,
            level: 0.0,
            overload: false,
            health: 100.0,
            workers: Vec::new(),
        }
    }

    pub fn add_worker(&mut self, worker: PowerUnit) {
        self.workers.push(worker);
    }

    pub fn absorb(&mut self) {
        let total_energy: f64 = self.workers.iter().map(|w| w.energy).sum();
        self.level += total_energy;

        for w in &mut self.workers {
            w.energy = 0.0;
        }

        if self.level > self.capacity {
            self.overload = true;
            let excess = self.level - self.capacity;
            self.health -= excess * 0.1;
        }
    }

    pub fn discharge(&mut self, amount: f64) {
        self.level = (self.level - amount).max(0.0);
        self.health = (self.health - amount * 0.01).max(0.0);

        if self.level <= self.capacity {
            self.overload = false;
        }
    }
}
