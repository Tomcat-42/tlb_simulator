#[cfg(test)]
mod tests;

use crate::mmu::{AddressTranslationResult, Mmu};
use crate::trace::Trace;

pub struct Simulation<const M: usize, const N: usize, const T: usize>
where
    [usize; usize::pow(2, M as u32) / usize::pow(2, N as u32)]: Sized,
{
    pub mmu: Mmu<M, N, T>,
    pub trace: Trace,
}

impl<const M: usize, const N: usize, const T: usize> Simulation<M, N, T>
where
    [usize; usize::pow(2, M as u32) / usize::pow(2, N as u32)]: Sized,
{
    type Error = Box<(dyn std::error::Error + 'static)>;
    type Result<R> = Result<R, Self::Error>;

    pub fn new(input: &str) -> Self::Result<Self> {
        let mmu = Mmu::new();
        let trace = input.parse::<Trace>()?;

        Ok(Self { mmu, trace })
    }

    pub fn run(&mut self) -> Self::Result<SimulationResult> {
        let (hits, misses, total) = self
            .trace
            .operations()
            .fold((0, 0, 0), |res, address| {
                match (self.mmu.translate(address), res) {
                    (AddressTranslationResult::Hit(_), (hits, misses, total)) => {
                        (hits + 1, misses, total + 1)
                    }
                    (AddressTranslationResult::Miss(_), (hits, misses, total)) => {
                        (hits, misses + 1, total + 1)
                    }
                }
            });

        // https://en.wikipedia.org/wiki/Translation_lookaside_buffer
        //
        //The average effective memory cycle rate is defined as [ m + ( 1 − p )h + pm] cycles,
        //where m is the number of cycles required for a memory read, p is the miss rate,
        //and h is the hit time in cycles.
        //If a TLB hit takes 1 clock cycle, a miss takes 30 clock cycles, a memory read takes 30 clock cycles,
        //and the miss rate is 1%, the effective memory cycle rate is an average of 30 + 0.99 × 1 + 0.01 × 30 (31.29 clock cycles per memory access)
        let cycle_rate = |miss_rate: f64| {
            move |mem_read_cycles: f64, hit_cycles: f64| {
                mem_read_cycles + (1.0 - miss_rate) * hit_cycles + miss_rate * mem_read_cycles
            }
        };

        Ok(SimulationResult::new(
            hits,
            misses,
            total,
            misses as f64 / total as f64,
            hits as f64 / total as f64,
            Box::new(cycle_rate(misses as f64 / total as f64)),
        ))
    }
}

pub struct SimulationResult {
    pub hits: usize,
    pub misses: usize,
    pub total: usize,
    pub miss_rate: f64,
    pub hit_rate: f64,
    pub effective_memory_cycle_rate: Box<dyn Fn(f64, f64) -> f64>,
}

impl SimulationResult {
    pub fn new(
        hits: usize,
        misses: usize,
        total: usize,
        miss_rate: f64,
        hit_rate: f64,
        effective_memory_cycle_rate: Box<dyn Fn(f64, f64) -> f64>,
    ) -> Self {
        Self {
            hits,
            misses,
            total,
            miss_rate,
            hit_rate,
            effective_memory_cycle_rate,
        }
    }
}
