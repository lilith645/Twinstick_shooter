use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct FirerateBuff {
  data: BuffData,
}

impl FirerateBuff {
  pub fn new(value: f32) -> FirerateBuff {
    FirerateBuff {
      data: BuffData::new(11, 5, LootRarity::Uncommon).set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> FirerateBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> FirerateBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for FirerateBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn apply_stat_modifiers(&self, _data: &mut StatModifier) {
    
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    let v = self.data().modified_value;
    if let Some(additive) = self.data().additive {
      let current_fire_speed = entity.weapon().firing_speed();
      if additive {
        entity.mut_weapon().set_firing_speed(current_fire_speed + v);
      } else {
        entity.mut_weapon().set_firing_speed(current_fire_speed * v);
      }
    } else {
      entity.mut_weapon().set_firing_speed(v);
    }
  }
  
  fn apply_to_bullet(&self, _bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
