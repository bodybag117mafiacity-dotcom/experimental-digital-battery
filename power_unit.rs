#[derive(Debug, Clone)]
pub struct PowerUnit {
    pub id: usize,            // identifiant du worker
    pub base_power: i32,      // puissance de base (10)
    pub pulse_power: i32,     // puissance actuelle
    pub energy: f64,          // énergie accumulée
    pub heat: f64,            // chaleur générée
    pub stability: f64,       // stabilité du worker
    pub cycles: u64,          // nombre de pulses effectués
}

impl PowerUnit {
    pub fn new(id: usize, base_power: i32) -> Self {
        Self {
            id,
            base_power,
            pulse_power: base_power,
            energy: 0.0,
            heat: 0.0,
            stability: 100.0,
            cycles: 0,
        }
    }

    pub fn pulse(&mut self) {
        self.cycles += 1;
        self.energy += self.pulse_power as f64;
        self.heat += self.pulse_power as f64 * 0.4;
        self.stability -= self.heat * 0.05;
    }

    pub fn boost(&mut self, amount: i32) {
        self.pulse_power += amount;
    }

    pub fn normalize(&mut self) {
        self.heat *= 0.5;
        self.stability = (self.stability + 10.0).min(100.0);
    }
}
