struct Character {
  name: String,
  build: Build,
  class: Class,
  level: u8,

  // stats
  strength: u8,
  vigor: u8,
  agility: u8,
  dexterity: u8,
  will: u8,
  knowledge: u8,
  ressourcefulness: u8,

  health: u8,
  memory_capacity: u8,
  utility_effectiveness: u8,
  luck: u8,
  health_recovery_bonus: u8,
  spell_recovery_bonus: u8,

  move_speed: u8,
  action_speed: u8,
  manual_dexterity: u8,
  spell_casting_speed: u8,
  equip_speed: u8,
  regular_interventions_speed: u8,
  magic_interaction_speed: u8,

  persuasiveness: u8,
  buff_duration: u8,
  debuff_duration: u8,

  armor_penetration: u8,
  magic_penetration: u8,

  headshot_reduction: u8,
  projectile_damage_reduction: u8,

  physical_damage_reduction: u8,
  magic_resistance: u8,
  physical_power_bonus: u8,
  magic_power_bonus: u8,
  primary_weapon: Weapon,
  secondary_weapon: Weapon,
  primary_weapon_impact_power: u8,
  secondary_weapon_impact_power: u8,
}