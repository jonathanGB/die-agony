use crate::direction::Direction;

/// Holds the value on each side of a 6-sided dice.
/// The values are optional, because we don't always know the value
/// on any side of the dice.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Dice {
    top: Option<i16>,
    bottom: Option<i16>,
    left: Option<i16>,
    right: Option<i16>,
    front: Option<i16>,
    back: Option<i16>,
}

impl Dice {
    /// Mutates the top value of the dice, and returns it.
    pub fn set_top(mut self, top: i16) -> Self {
        self.top = Some(top);

        self
    }

    /// Creates a new dice, based on a rotation in the given direction of the current dice.
    pub fn roll_in(&self, direction: Direction) -> Self {
        match direction {
            Direction::UP => self.roll_up(),
            Direction::RIGHT => self.roll_right(),
            Direction::DOWN => self.roll_down(),
            Direction::LEFT => self.roll_left(),
        }
    }

    pub fn roll_up(&self) -> Self {
        Self {
            top: self.back,
            bottom: self.front,
            left: self.left,
            right: self.right,
            front: self.top,
            back: self.bottom,
        }
    }

    pub fn roll_down(&self) -> Self {
        Self {
            top: self.front,
            bottom: self.back,
            left: self.left,
            right: self.right,
            front: self.bottom,
            back: self.top,
        }
    }

    pub fn roll_left(&self) -> Self {
        Self {
            top: self.right,
            bottom: self.left,
            left: self.top,
            right: self.bottom,
            front: self.front,
            back: self.back,
        }
    }

    pub fn roll_right(&self) -> Self {
        Self {
            top: self.left,
            bottom: self.right,
            left: self.bottom,
            right: self.top,
            front: self.front,
            back: self.back,
        }
    }

    /// Returns the value on top of the dice, if any.
    pub fn get_top(&self) -> Option<i16> {
        self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_default_dice() -> Dice {
        Dice {
            top: Some(1),
            bottom: Some(3),
            left: Some(4),
            right: Some(5),
            front: Some(0),
            back: Some(2),
        }
    }

    #[test]
    fn roll_in_works() {
        let dice = create_default_dice();
        assert_eq!(dice.roll_in(Direction::UP), dice.roll_up());
        assert_eq!(dice.roll_in(Direction::RIGHT), dice.roll_right());
        assert_eq!(dice.roll_in(Direction::DOWN), dice.roll_down());
        assert_eq!(dice.roll_in(Direction::LEFT), dice.roll_left());
    }

    #[test]
    fn roll_left_works() {
        let dice = create_default_dice();
        assert_eq!(
            dice.roll_left(),
            Dice {
                top: Some(5),
                bottom: Some(4),
                left: Some(1),
                right: Some(3),
                front: Some(0),
                back: Some(2),
            }
        )
    }

    #[test]
    fn roll_right_works() {
        let dice = create_default_dice();
        assert_eq!(
            dice.roll_right(),
            Dice {
                top: Some(4),
                bottom: Some(5),
                left: Some(3),
                right: Some(1),
                front: Some(0),
                back: Some(2),
            }
        )
    }

    #[test]
    fn roll_up_works() {
        let dice = create_default_dice();
        assert_eq!(
            dice.roll_up(),
            Dice {
                top: Some(2),
                bottom: Some(0),
                left: Some(4),
                right: Some(5),
                front: Some(1),
                back: Some(3),
            }
        )
    }

    #[test]
    fn roll_down_works() {
        let dice = create_default_dice();
        assert_eq!(
            dice.roll_down(),
            Dice {
                top: Some(0),
                bottom: Some(2),
                left: Some(4),
                right: Some(5),
                front: Some(3),
                back: Some(1),
            }
        )
    }

    #[test]
    fn complex_roll_works() {
        let dice = create_default_dice();
        assert_eq!(
            dice.roll_down()
                .roll_left()
                .roll_right()
                .roll_right()
                .roll_up()
                .roll_up()
                .roll_right(),
            Dice {
                top: Some(2),
                bottom: Some(0),
                left: Some(4),
                right: Some(5),
                front: Some(1),
                back: Some(3),
            }
        );
    }

    #[test]
    fn roll_left_4_times_back_to_init() {
        let dice = create_default_dice();

        let rolled_die = dice.roll_left();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_left();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_left();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_left();
        assert_eq!(dice, rolled_die);
    }

    #[test]
    fn roll_right_4_times_back_to_init() {
        let dice = create_default_dice();

        let rolled_die = dice.roll_right();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_right();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_right();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_right();
        assert_eq!(dice, rolled_die);
    }

    #[test]
    fn roll_up_4_times_back_to_init() {
        let dice = create_default_dice();

        let rolled_die = dice.roll_up();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_up();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_up();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_up();
        assert_eq!(dice, rolled_die);
    }

    #[test]
    fn roll_down_4_times_back_to_init() {
        let dice = create_default_dice();

        let rolled_die = dice.roll_down();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_down();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_down();
        assert_ne!(dice, rolled_die);

        let rolled_die = rolled_die.roll_down();
        assert_eq!(dice, rolled_die);
    }

    #[test]
    fn roll_left_and_right_back_to_init() {
        let dice = create_default_dice();
        assert_eq!(dice, dice.roll_left().roll_right());
    }

    #[test]
    fn roll_right_and_left_back_to_init() {
        let dice = create_default_dice();
        assert_eq!(dice, dice.roll_right().roll_left());
    }

    #[test]
    fn roll_up_and_backward_back_to_init() {
        let dice = create_default_dice();
        assert_eq!(dice, dice.roll_up().roll_down());
    }

    #[test]
    fn roll_down_and_forward_back_to_init() {
        let dice = create_default_dice();
        assert_eq!(dice, dice.roll_down().roll_up());
    }

    #[test]
    fn top_accessors_work() {
        assert!(Dice::default().get_top().is_none());

        let mut dice = create_default_dice();
        let new_top = 42;
        dice.top = Some(new_top);

        let top = dice.get_top();
        assert_eq!(top, Some(new_top));

        let new_top = 27;
        assert_eq!(dice.set_top(new_top).get_top(), Some(new_top));
    }
}
