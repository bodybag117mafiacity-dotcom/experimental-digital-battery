// tests/energy_tests.rs

use virtual_digital_battery::{PowerUnit, DigitalBattery};

//
// LISTE DES TESTS À FAIRE POUR LE SYSTÈME ÉNERGÉTIQUE
//
// Chaque test valide une partie du moteur :
// - workers (units)
// - pulses
// - chaleur
// - stabilité
// - surcharge
// - décharge
// - multi-workers
// - batterie complète
// - intégration
// - plugin 3D (optionnel)
//

#[test]
fn test_single_worker_pulse() {
    let mut worker = PowerUnit::new(1, 10);
    worker.pulse();
    assert!(worker.energy > 0.0);
}

#[test]
fn test_worker_heat_increases() {
    let mut worker = PowerUnit::new(1, 10);
    worker.pulse();
    assert!(worker.heat > 0.0);
}

#[test]
fn test_worker_stability_decreases() {
    let mut worker = PowerUnit::new(1, 10);
    worker.pulse();
    assert!(worker.stability < 100.0);
}

#[test]
fn test_worker_boost() {
    let mut worker = PowerUnit::new(1, 10);
    worker.boost(5);
    assert_eq!(worker.pulse_power, 15);
}

#[test]
fn test_worker_normalize() {
    let mut worker = PowerUnit::new(1, 10);
    worker.pulse();
    worker.normalize();
    assert!(worker.heat < 10.0);
}

#[test]
fn test_battery_absorb_single_worker() {
    let mut worker = PowerUnit::new(1, 10);
    let mut battery = DigitalBattery::new(100.0);

    worker.pulse();
    battery.add_worker(worker.clone());
    battery.absorb();

    assert!(battery.level > 0.0);
}

#[test]
fn test_battery_absorb_multiple_workers() {
    let mut battery = DigitalBattery::new(200.0);

    for id in 0..5 {
        let mut w = PowerUnit::new(id, 10);
        w.pulse();
        battery.add_worker(w);
    }

    battery.absorb();
    assert!(battery.level > 10.0);
}

#[test]
fn test_battery_overload() {
    let mut battery = DigitalBattery::new(20.0);

    let mut w = PowerUnit::new(1, 50);
    w.pulse();
    battery.add_worker(w);

    battery.absorb();
    assert!(battery.overload);
}

#[test]
fn test_battery_discharge() {
    let mut battery = DigitalBattery::new(100.0);

    let mut w = PowerUnit::new(1, 10);
    w.pulse();
    battery.add_worker(w);

    battery.absorb();
    battery.discharge(20.0);

    assert!(battery.level <= 80.0);
}

#[test]
fn test_battery_health_decreases_on_overload() {
    let mut battery = DigitalBattery::new(10.0);

    let mut w = PowerUnit::new(1, 50);
    w.pulse();
    battery.add_worker(w);

    battery.absorb();
    assert!(battery.health < 100.0);
}

//
// OPTIONAL: TEST PLUGIN 3D
//
// Ce test vérifie simplement que le plugin peut être appelé.
// Le plugin 3D n'est pas encore codé, mais on prépare le test.
//
#[test]
fn test_plugin_3d_integration() {
    // Placeholder: on vérifiera plus tard que le plugin 3D
    // peut recevoir les données de la batterie et les workers.
    let plugin_loaded = true;
    assert!(plugin_loaded);
}
