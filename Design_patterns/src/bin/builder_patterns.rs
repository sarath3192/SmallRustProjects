// https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
// https://blog.ediri.io/design-patterns-in-rust-an-introduction-to-the-builder-pattern

// Rust 🦀 does not allow function overloading, so we can't create a new new method for this or any further cases

// Construct an object with calls to a builder helper.

/**** Without builder pattern ****


#[derive(Debug, Clone)]
struct Nodes {
    name: String,
    size: String,
    count: u32,
}

#[derive(Debug)]
pub struct KubernetesCluster {
    name: String,
    version: String,
    auto_upgrade: bool,
    node_pool: Option<Vec<Nodes>>,
}

impl KubernetesCluster {
      fn new(name: String, version: String) -> Self {
          Self {
              name,
              version,
              auto_upgrade: false,
              node_pool: None,
          }
      }
      fn new_with_auto_upgrade(name: String, version: String, auto_upgrade: bool) -> Self {
          Self {
              name,
              version,
              auto_upgrade,
              node_pool: None,
          }
      }
      fn new_complete(
          name: String,
          version: String,
          auto_upgrade: bool,
          node_pool: Option<Vec<Nodes>>,
      ) -> Self {
          Self {
              name,
              version,
              auto_upgrade,
              node_pool,
          }
      }
}

fn main() {
      let name = "my-cluster".to_owned();
      let version = "1.25.0".to_owned();
  
      let nodes = vec![
          Nodes {
              name: "node-1".to_owned(),
              size: "small".to_owned(),
              count: 1,
          }
      ];
  
      let basic_cluster = KubernetesCluster::new(name.clone(), version.clone());
  
      let auto_upgrade_cluster = KubernetesCluster::new_with_auto_upgrade(
          name.clone(),
          version.clone(),
          true,
      );
  
      let complete_cluster = KubernetesCluster::new_complete(
          name.clone(),
          version.clone(),
          true,
          Some(nodes),
      );
  } ***/
  

/* with builder pattern 


#[derive(Debug, Clone)]
struct Nodes {
    name: String,
    size: String,
    count: u32,
}

#[derive(Debug)]
pub struct KubernetesCluster {
    name: String,
    version: String,
    auto_upgrade: bool,
    node_pool: Option<Vec<Nodes>>,
}

#[derive(Debug)]
pub struct KubernetesClusterBuilder {
    name: String,
    version: String,
    auto_upgrade: Option<bool>,
    node_pool: Option<Vec<Nodes>>,
}

impl KubernetesClusterBuilder {
    fn auto_upgrade(&mut self, auto_upgrade: bool) -> &mut Self {
        self.auto_upgrade = Some(auto_upgrade);
        self
    }

    fn node_pool(&mut self, node_pool: Vec<Nodes>) -> &mut Self {
        self.node_pool = Some(node_pool);
        self
    }

    fn build(&mut self) -> KubernetesCluster {
        KubernetesCluster {
            name: self.name.clone(),
            version: self.version.clone(),
            auto_upgrade: self.auto_upgrade.unwrap_or_default(),
            node_pool: self.node_pool.clone(),
        }
    }
}

impl KubernetesCluster {
    fn new(name: String, version: String) -> KubernetesClusterBuilder {
        KubernetesClusterBuilder {
            name,
            version,
            auto_upgrade: None,
            node_pool: None,
        }
    }
}

fn main() {
    let name = "my-cluster".to_owned();
    let version = "1.25.0".to_owned();

    let nodes = vec![
        Nodes {
            name: "node-1".to_owned(),
            size: "small".to_owned(),
            count: 1,
        }
    ];

    let mut basic_cluster = KubernetesCluster::new(name.clone(), version.clone());

    println!("{:?}",basic_cluster.auto_upgrade(true).node_pool(nodes.clone()));

    let auto_upgrade_cluster = KubernetesCluster::new(
        name.clone(),
        version.clone(),
    ).auto_upgrade(true)
        .build();

    let complete_cluster = KubernetesCluster::new(
        name.clone(),
        version.clone(),
    ).auto_upgrade(true)
        .node_pool(nodes)
        .build();
}
*/

use std::default;

#[derive(Debug)]
struct Car {
    color: Option<String>,
    model: Option<String>,
    doors: Option<usize>,
}

struct Carbuilder {
    color: Option<String>,
    model: Option<String>,
    doors: Option<usize>,
}

impl Carbuilder {
    fn color(&mut self, color: String) -> &mut Self{
       self.color = Some(color);
       self
    }
    
    fn model(&mut self, model: String) -> &mut Self{
        self.model = Some(model);
        self
    }

    fn doors(&mut self, model: usize) -> &mut Self{
        self.doors = Some(model);
        self
    }

    fn build(&mut self) -> Car{
       Car{
        color: self.color.clone(),
        model: self.model.clone(),
        doors: self.doors.clone(),
        }
    }
}

impl Car {
    fn new() -> Carbuilder {
     Carbuilder{
        color: None,
        model: None,
        doors: None,
       
     }
    }
}

fn main(){
    let Bmw = Car::new()
                               .color(String::from("Yellow"))
                               .model(String::from("s23"))
                               .build();
    let tesla = Car::new()
                .color(String::from("Red"))
                .model(String::from("2.0"))
                .doors(4)
                .build();
}

