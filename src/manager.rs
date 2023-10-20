use super::agent::Agent;

/// The struct used to spawn and control agents.
/// In most applications, this will be the main, and possibly only, interface to the library.
pub struct SwarmManager<A: Agent> {
    agents: Vec<A>,
}

impl<A: Agent> SwarmManager<A> {
    /// Constructs a new, empty `SwarmManager`.
    ///
    /// # Examples
    /// ```
    /// use sassi::{SwarmManager};
    /// # use sassi::MyAgent;
    /// let mut manager: SwarmManager<MyAgent> = SwarmManager::new();
    /// ```
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Constructs a new, empty `SwarmManager` with a given capacity.
    ///
    /// The constructed `SwarmManager` will hold at least `capacity` items before reallocation occurs.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { agents: Vec::with_capacity(capacity) }
    }

    /// Constructs a new `SwarmManager` containing the given collection of elements.
    ///
    /// The collection is consumed and the `SwarmManager` takes ownership of the elements within it.
    pub fn from(agents: impl Into<Vec<A>>) -> Self {
        Self { agents: agents.into() }
    }

    /// Constructs a new [Agent] using [Agent::spawn_with]
    /// Args is defined by the user in their implementation of [Agent]
    ///
    /// # Examples
    ///
    /// ```
    /// # use sassi::{Agent,SwarmManager};
    /// # #[derive(Debug, Default)]
    /// struct MyAgent(ArgStruct);
    /// # #[derive(Debug, PartialEq, Clone, Default)]
    /// struct ArgStruct(u8, u8);
    /// impl Agent for MyAgent {
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

    /// Gives control of an agent constructed elsewhere to the Manager.
    ///
    /// Consumes the given agent.
    pub fn add_agent(&mut self, agent: A) {
        self.agents.push(agent);
    }
}

impl<A: Agent> Default for SwarmManager<A> {
    fn default() -> Self {
        Self::new()
    }
}
