//! Logic for finding and eating food when the [`EnergyPool`] is low.

use bevy::prelude::*;

use crate::{
    asset_management::manifest::{Id, Item, ItemManifest, Unit, UnitManifest},
    organisms::energy::{Energy, EnergyPool},
};

use super::goals::Goal;

/// The item(s) that a unit must consume to gain [`Energy`].
#[derive(Component, Clone, Debug)]
pub(crate) struct Diet {
    /// The item that must be eaten
    item: Id<Item>,
    /// The amount of energy restored per item destroyed
    energy: Energy,
}

impl Diet {
    /// Creates a new [`Diet`] component.
    pub(crate) fn new(item: Id<Item>, energy: Energy) -> Self {
        Diet { item, energy }
    }

    /// The type of item that this unit must consume.
    pub(crate) fn item(&self) -> Id<Item> {
        self.item
    }

    /// The amount of [`Energy`] gained when a single item of the correct type is consumed.
    pub(crate) fn energy(&self) -> Energy {
        self.energy
    }

    /// Pretty formatting for this type
    pub(crate) fn display(&self, item_manifest: &ItemManifest) -> String {
        format!(
            "{} -> {} energy",
            item_manifest.name(self.item),
            self.energy
        )
    }
}

/// Swaps the goal to [`Goal::Eat`] when energy is low
pub(super) fn check_for_hunger(
    mut unit_query: Query<(&mut Goal, &EnergyPool, &Id<Unit>)>,
    unit_manifest: Res<UnitManifest>,
) {
    for (mut goal, energy_pool, unit_id) in unit_query.iter_mut() {
        if energy_pool.is_hungry() {
            let diet = &unit_manifest.get(*unit_id).diet;
            *goal = Goal::Eat(diet.item);
        } else if matches!(*goal, Goal::Eat(..)) && energy_pool.is_satiated() {
            *goal = Goal::Wander {
                remaining_actions: None,
            }
        }
    }
}
