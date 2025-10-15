pub mod actor;
pub mod assets;
pub mod systems;

use bevy_ecs::{
    schedule::{ExecutorKind, Schedule},
    world::World,
};

pub struct Simulation {
    world: World,
    schedule: Schedule,
}

impl Simulation {
    pub fn new() -> Self {
        let mut schedule = Schedule::default();
        schedule.set_executor_kind(ExecutorKind::MultiThreaded);

        Self {
            world: World::default(),
            schedule,
        }
    }

    pub fn step(&mut self) {
        // Update the simulation state here
        self.schedule.run(&mut self.world);
    }
}
