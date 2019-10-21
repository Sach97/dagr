use daggy::stabledag::{NodeIndex, StableDag};
use petgraph::visit::IntoNodeReferences;
use std::io::Error;

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

    pub fn get_index(&mut self, name: &str) -> usize {
        self.dag
            .graph()
            .node_references()
            .map(|(ix, weight)| {
                if weight.name == name {
                    Ok(ix.index())
                } else {
                    Err(())
                }
            })
            .collect::<Vec<Result<usize, ()>>>()
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<usize>>()
            .first()
            .unwrap()
            .to_owned()
    }

    pub fn add_edges(
        &mut self,
        relationships: Vec<(&'static str, &'static str)>,
        relationship: &str,
    ) {
        for (name1, name2) in relationships {
            self.add_edge(name1, name2, relationship);
        }
    }
    pub fn add_edge(&mut self, name1: &str, name2: &str, relationship: &str) {
        let idx1 = self.get_index(name1);
        let idx2 = self.get_index(name2);
        self.dag.add_edge(
            NodeIndex::new(idx1),
            NodeIndex::new(idx2),
            relationship.to_owned(),
        );
    }
}
