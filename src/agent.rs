/// This trait provides an interface for the SwarmManager to use.
/// N is the size of the state array returned by the agent.
pub trait SwarmAgent: Default {
    const STATE_SIZE: usize;
    /// Decisions made in a format usable by the agent
    type Input;

    /// Contains parameters required to construct a new SwarmAgent
    type SpawnArgs;

    /// Return an array of floats representing the state
    fn get_state(&self) -> [f32; Self::STATE_SIZE];
    /// Act based on recieved input
    // TODO: allow returning some kind of cost evaluation?
    fn evaluate(&mut self, input: Self::Input);

    fn spawn_new(args: &Self::SpawnArgs) -> Self;
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
    impl SwarmAgent for TestAgent {
        const STATE_SIZE: usize = 3;
        type Input = (f32, f32, Discrete);
        type SpawnArgs = ();
        fn get_state(&self) -> [f32; Self::STATE_SIZE] {
            [self.state.0, self.state.1, self.state.2.into()]
        }
        fn evaluate(&mut self, input: Self::Input) {
            self.state = input;
        }
        fn spawn_new(_args: &Self::SpawnArgs) -> Self {
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
        // assert_eq!(a.get_state(), [i.0, i.1, i.2.into()]);
    }
}
