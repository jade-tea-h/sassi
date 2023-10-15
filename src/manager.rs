use super::agent::SwarmAgent;

/// The struct used to spawn and control agents.
/// This is where all of the functionality of the crate is.
pub struct SwarmManager<A: SwarmAgent> {
    agents: Vec<A>,
}

impl<A: SwarmAgent> SwarmManager<A> {
    /// New empty SwarmManager
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Constructs a new [SwarmAgent] using [SwarmAgent::spawn_new]
    /// Args is defined by the user in their implementation of SwarmAgent
    ///
    /// # Examples
    ///
    /// ```
    /// # use sassi::{SwarmAgent,SwarmManager};
    /// # #[derive(Debug, Default)]
    /// struct MyAgent(ArgStruct);
    /// # #[derive(Debug, PartialEq, Clone, Default)]
    /// struct ArgStruct(u8, u8);
    /// impl SwarmAgent for MyAgent {
    ///     type SpawnArgs = ArgStruct;
    ///     // ...
    ///     # const STATE_SIZE: usize = 0;
    ///     # type Input = [u8;0];
    ///     fn spawn_with(args: &Self::SpawnArgs) -> Self { MyAgent(args.clone()) }
    ///     # fn get_state(&self) -> [f32;0] { [] }
    ///     # fn evaluate(&mut self, input: Self::Input) {}
    /// }
    /// let mut manager = SwarmManager::<MyAgent>::new();
    /// let arg = ArgStruct(1, 2);
    /// let id = manager.spawn_agent(&arg.clone());
    /// assert_eq!(arg, manager.agent(id).unwrap().0);
    /// ```
    pub fn spawn_agent(&mut self, args: &A::SpawnArgs) -> usize {
        let new = A::spawn_with(args);
        self.agents.push(new);
        self.agents.len() - 1
    }

    /// Get agent
    pub fn agent(&self, id: usize) -> Option<&A> {
        self.agents.get(id)
    }
}

impl<A: SwarmAgent> Default for SwarmManager<A> {
    fn default() -> Self {
        Self::new()
    }
}
