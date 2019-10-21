use daggy::petgraph::visit::{
    EdgeRef, IntoEdgeReferences, IntoEdges, IntoNodeReferences, NodeIndexable,
};
use daggy::stabledag::{EdgeIndex, NodeIndex, Parents, StableDag, Walker};

#[derive(Debug, Clone)]
pub struct Weight {
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

    pub fn find_relationship(&mut self, name1: &str, name2: &str) -> String {
        //parent of Thor is Odin
        let ix1 = self.get_index(name1);
        let ix2 = self.get_index(name2);
        let result = self
            .dag
            .find_edge(self.dag.from_index(ix1), self.dag.from_index(ix2));
        match result {
            Some(edge) => self.dag.edge_weight(edge).unwrap().to_owned(),
            None => panic!("error"), //TODO: handle error
        }
    }

    pub fn find_children(&mut self, name: &str) -> Vec<Weight> {
        //find all thor children
        let ix = self.get_index(name);
        // let result = self.dag.recursive_walk(self.dag.from_index(ix), |&g, n| {
        //     Some((
        //         g.find_edge(g, self.dag.from_index(n))
        //             .unwrap(),
        //         n,
        //     ))
        // });
        let mut child_walker = self.dag.children(self.dag.from_index(ix));
        let child1 = child_walker.walk_next(&self.dag).map(|(e, _)| e).unwrap();
        let child2 = child_walker.walk_next(&self.dag).map(|(e, _)| e).unwrap();
        let child3 = child_walker.walk_next(&self.dag).map(|(e, _)| e).unwrap();
        vec![
            self.dag
                .node_weight(self.dag.edge_endpoints(child1).unwrap().1)
                .unwrap()
                .to_owned(),
            self.dag
                .node_weight(self.dag.edge_endpoints(child2).unwrap().1)
                .unwrap()
                .to_owned(), //self.dag.edge_endpoints(child1).unwrap().0;
        ]
    }
}
