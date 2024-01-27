struct Chest {
  requirement: Vec<Class>,
  armor_rating: u8,
}

impl Chest {
  fn new(requirement: Vec<Class>, armor_rating: u8) -> Self {
    Self {
      requirement,
      armor_rating,
    }
  }
}
