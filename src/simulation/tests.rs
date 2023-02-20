use super::*;

#[test]
fn simulation_should_run() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let trace = "07b243a0 R
            07b24380 R";

    let mut simulation: Simulation<32, 12, 4> = Simulation::new(trace)?;
    let SimulationResult {
        hits,
        misses,
        total,
        hit_rate,
        miss_rate,
        effective_memory_cycle_rate,
    } = simulation.run()?;

    assert_eq!(hits, 1);
    assert_eq!(misses, 1);
    assert_eq!(total, 2);
    assert_eq!(hit_rate, 0.5);
    assert_eq!(miss_rate, 0.5);
    assert_eq!(effective_memory_cycle_rate(30.0, 1.0), 45.5);

    Ok(())
}
