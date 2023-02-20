#![allow(incomplete_features)] // 💀
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]
#![feature(box_syntax)]
mod mmu;
mod simulation;
mod trace;

pub use mmu::Mmu;
pub use simulation::{Simulation, SimulationResult};
pub use trace::Trace;
