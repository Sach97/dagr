use daggy::stabledag::{NodeIndex, StableDag};

#[derive(Debug)]
struct Weight {
    name: String,
    species: String,
    gender: String,
}

impl Weight {
    fn new(name: &'static str, species: &str, gender: &str) -> Weight {
        Weight {
            name: name.to_owned(),
            species: species.to_owned(),
            gender: gender.to_owned(),
        }
    }
}
#[derive(Debug)]
pub struct Dagr {
    dag: StableDag<Weight, String, u32>,
}

impl Dagr {
    pub fn new() -> Dagr {
        Dagr {
            dag: StableDag::<Weight, String, u32>::new(),
        }
    }

    pub fn add_nodes(&mut self, nodes: Vec<(&'static str, &'static str)>, species: &str) {
        for (name, gender) in nodes {
            self.dag.add_node(Weight::new(name, species, gender));
        }
    }

    // pub fn add_eges(&mut self, edges: Vec<(&'static str, &'static str)>) {
    //     for (name, gender) in edges {
    //         assert!(self
    //             .dag
    //             .add_edge(
    //                self.dag.nodes,
    //                 NodeIndex::new(calculate_hash(&gender.to_owned())),
    //                 0
    //             )
    //             .is_ok());
    //     }
    // }
    pub fn add_edge(&mut self, idx1: usize, idx2: usize, relationship: &str) {
        self.dag.add_edge(
            NodeIndex::new(idx1),
            NodeIndex::new(idx2),
            relationship.to_owned(),
        );
    }
}
