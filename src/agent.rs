/// An `Agent` is a single acting agent in a swarm.
/// The
pub trait Agent {
    /// Size of the state array returned as input for the decision model used by [SwarmManager].
    const STATE_SIZE: usize;

    /// Decisions made in a format usable by the agent.
    ///
    /// Should probably be a tuple of primitives and enums.
    /// Most likely will be somehow restricted somehow in the future.
    type Input;

    /// Contains parameters required to construct a new `Agent`.
    type SpawnArgs;

    /// Return an array of floats representing the state.
    fn get_state(&self) -> [f32; Self::STATE_SIZE];

    /// Act based on recieved input.
    // TODO: allow returning some kind of cost evaluation?
    fn evaluate(&mut self, input: Self::Input);

    /// Construct new generic `Agent`
    fn spawn() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
    // fn spawn() -> Self
    // where
    //     // Self: ?Default,
    //     Self: Sized,
    //     Self::SpawnArgs: Default,
    // {
    //     Self::spawn_with(&Self::SpawnArgs::default())
    // }

    /// Construct new `Agent` using [Self::SpawnArgs].
    fn spawn_with(args: &Self::SpawnArgs) -> Self;
}

#[cfg(test)]
mod test {
    use super::*;
    #[derive(Debug, PartialEq, Clone, Copy, Default)]
    enum Discrete {
        On,
        #[default]
        Off,
    }
    impl Into<f32> for Discrete {
        fn into(self) -> f32 {
            match self {
                Self::On => 1.0,
                Self::Off => 0.0,
            }
        }
    }
    impl From<f32> for Discrete {
        fn from(f: f32) -> Discrete {
            match f.ceil() as i16 {
                1 => Discrete::On,
                _ => Discrete::Off,
            }
        }
    }
    #[derive(Default)]
    struct TestAgent {
        state: (f32, f32, Discrete),
    }
    impl Agent for TestAgent {
        const STATE_SIZE: usize = 3;
        type Input = (f32, f32, Discrete);
        type SpawnArgs = ();
        fn get_state(&self) -> [f32; Self::STATE_SIZE] {
            [self.state.0, self.state.1, self.state.2.into()]
        }
        fn evaluate(&mut self, input: Self::Input) {
            self.state = input;
        }
        fn spawn_with(_args: &Self::SpawnArgs) -> Self {
            Self::default()
        }
    }

    #[test]
    fn agent_test() {
        let s = [0.0, 0.0, Discrete::Off.into()];
        let mut a = TestAgent {
            state: (s[0], s[1], s[2].into()),
        };
        assert_eq!(a.get_state(), s);
        let i = (1.0, 1.0, Discrete::On);
        a.evaluate(i.clone());
        assert_eq!(a.state, i);
    }
}
