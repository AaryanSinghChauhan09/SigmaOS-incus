//! incus model — system container/VM manager (LXD fork).

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub type_: String,
    pub state: String,
    pub image: String,
}

#[derive(Debug, Clone)]
pub struct Model { pub instances: Vec<Instance> }
impl Default for Model { fn default() -> Self { Self { instances: vec![] } } }

impl Model {
    pub fn new() -> Self { Self::default() }
    pub fn launch(&mut self, name: &str, image: &str, type_: &str) -> bool {
        self.instances.push(Instance { name: name.into(), type_: type_.into(), state: "Running".into(), image: image.into() });
        true
    }
    pub fn list(&self) -> &[Instance] { &self.instances }
    pub fn stop(&mut self, name: &str) -> bool {
        if let Some(i) = self.instances.iter_mut().find(|i| i.name == name) { i.state = "Stopped".into(); true } else { false }
    }
    pub fn delete(&mut self, name: &str) -> bool {
        if let Some(pos) = self.instances.iter().position(|i| i.name == name) { self.instances.remove(pos); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn launch_delete() {
        let mut i = Model::new();
        assert!(i.launch("vm1", "ubuntu", "virtual-machine"));
        assert!(i.delete("vm1"));
        assert!(i.list().is_empty());
    }
}
