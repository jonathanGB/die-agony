#[derive(Debug, Default, PartialEq)]
struct Die {
    top: Option<i8>,
    bottom: Option<i8>,
    left: Option<i8>,
    right: Option<i8>,
    front: Option<i8>,
    back: Option<i8>,
}

impl Die {
    pub fn roll_forward(&self) -> Self {
        Self {
            top: self.back,
            bottom: self.front,
            left: self.left,
            right: self.right,
            front: self.top,
            back: self.bottom,
        }
    }

    pub fn roll_backward(&self) -> Self {
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

    pub fn get_top(&self) -> &Option<i8> {
        &self.top
    }

    pub fn set_top(&mut self, top: Option<i8>) {
        self.top = top;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_default_die() -> Die {
        Die {
            top: Some(1),
            bottom: Some(3),
            left: Some(4),
            right: Some(5),
            front: Some(0),
            back: Some(2),
        }
    }

    #[test]
    fn roll_left_works() {
        let die = create_default_die();
        assert_eq!(
            die.roll_left(),
            Die {
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
        let die = create_default_die();
        assert_eq!(
            die.roll_right(),
            Die {
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
    fn roll_forward_works() {
        let die = create_default_die();
        assert_eq!(
            die.roll_forward(),
            Die {
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
    fn roll_backward_works() {
        let die = create_default_die();
        assert_eq!(
            die.roll_backward(),
            Die {
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
        let die = create_default_die();
        assert_eq!(
            die.roll_backward()
                .roll_left()
                .roll_right()
                .roll_right()
                .roll_forward()
                .roll_forward()
                .roll_right(),
            Die {
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
        let die = create_default_die();

        let rolled_die = die.roll_left();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_left();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_left();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_left();
        assert_eq!(die, rolled_die);
    }

    #[test]
    fn roll_right_4_times_back_to_init() {
        let die = create_default_die();

        let rolled_die = die.roll_right();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_right();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_right();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_right();
        assert_eq!(die, rolled_die);
    }

    #[test]
    fn roll_forward_4_times_back_to_init() {
        let die = create_default_die();

        let rolled_die = die.roll_forward();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_forward();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_forward();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_forward();
        assert_eq!(die, rolled_die);
    }

    #[test]
    fn roll_backward_4_times_back_to_init() {
        let die = create_default_die();

        let rolled_die = die.roll_backward();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_backward();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_backward();
        assert_ne!(die, rolled_die);

        let rolled_die = rolled_die.roll_backward();
        assert_eq!(die, rolled_die);
    }

    #[test]
    fn roll_left_and_right_back_to_init() {
        let die = create_default_die();
        assert_eq!(die, die.roll_left().roll_right());
    }

    #[test]
    fn roll_right_and_left_back_to_init() {
        let die = create_default_die();
        assert_eq!(die, die.roll_right().roll_left());
    }

    #[test]
    fn roll_forward_and_backward_back_to_init() {
        let die = create_default_die();
        assert_eq!(die, die.roll_forward().roll_backward());
    }

    #[test]
    fn roll_backward_and_forward_back_to_init() {
        let die = create_default_die();
        assert_eq!(die, die.roll_backward().roll_forward());
    }

    #[test]
    fn top_accessors_work() {
        assert!(Die::default().get_top().is_none());

        let mut die = create_default_die();

        let top = die.get_top();
        assert_eq!(top, &die.top);

        let new_top = Some(27);
        die.set_top(new_top);
        assert_eq!(die.get_top(), &new_top);
    }
}
