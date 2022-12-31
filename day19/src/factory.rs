use crate::blueprint::Robot;
use crate::Blueprint;
use crate::Resources;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    resources: Resources,
    income: Resources,
    time_remaining: u32,
    min_geodes: u32,
    max_geodes: u32,
}

impl State {
    const fn new(resources: Resources, income: Resources, time_remaining: u32) -> Self {
        // min assumes never building a geode-cracking robot
        let min_geodes = resources.geode + income.geode * time_remaining;
        // max assumes building a building a geode-cracking robot every turn
        let max_geodes = min_geodes + time_remaining * time_remaining.saturating_sub(1) / 2;
        Self {
            resources,
            income,
            time_remaining,
            min_geodes,
            max_geodes,
        }
    }

    fn build(&self, robot: Robot) -> Option<Self> {
        let time = robot
            .cost
            .saturating_sub(self.resources)
            .checked_div_ceil(self.income)?
            + 1;
        if self.time_remaining < time {
            None
        } else {
            let resources = self.resources + time * self.income - robot.cost;
            let income = self.income + robot.collection;
            let time_remaining = self.time_remaining - time;
            Some(Self::new(resources, income, time_remaining))
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.max_geodes
            .cmp(&other.max_geodes)
            .then(self.min_geodes.cmp(&other.min_geodes))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn max_geodes(blueprint: Blueprint, time: u32) -> u32 {
    let mut max = 0;
    let mut states = BinaryHeap::new();
    states.push(State::new(Resources::ZERO, Resources::ORE, time));
    while let Some(state) = states.pop() {
        if state.max_geodes <= max {
            break;
        }
        for robot in blueprint.robots {
            if state.income.ore < blueprint.max_cost.ore || robot.collection.ore == 0 {
                if let Some(state) = state.build(robot) {
                    if state.max_geodes > max {
                        max = max.max(state.min_geodes);
                        states.push(state);
                    }
                }
            }
        }
    }
    max
}
