/// Contains structures for node object
///

pub struct Node {
    name: String,
}

impl Node {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn show_name(&self) {
        println!("name: {}", self.name);
    }

    pub fn get_distance(&self) {
        println!("distance of wifi: {} m", self.name)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let node = Node::new(String::from("John"));
        assert_eq!(node.get_name(), String::from("John"));
    }
}

