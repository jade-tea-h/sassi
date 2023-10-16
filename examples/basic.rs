use sassi::{Agent, SwarmManager};

#[derive(Debug, PartialEq, Clone)]
enum AgentInput {
    Discrete(bool),
    Fuzzy(f32),
}
#[derive(Debug, PartialEq, Clone, Default)]
struct MyAgent {
    data: Vec<f32>,
}
impl Agent for MyAgent {
    const STATE_SIZE: usize = 5;
    type Input = [AgentInput; 2];
    type SpawnArgs = Vec<f32>;
    fn spawn_with(args: &Self::SpawnArgs) -> Self {
        MyAgent { data: args.clone() }
    }
    fn get_state(&self) -> [f32; Self::STATE_SIZE] {
        let mut state: [f32; Self::STATE_SIZE] = [0.0; Self::STATE_SIZE];
        for i in 0..Self::STATE_SIZE {
            if i < self.data.len() {
                state[i] = self.data[i];
            } else {
                state[i] = 0.0;
            }
        }
        state
    }
    fn evaluate(&mut self, input: Self::Input) {
        todo!()
    }
}

fn main() {
    let manager = SwarmManager::<MyAgent>::new();
}
