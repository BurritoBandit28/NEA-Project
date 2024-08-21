#[derive(Debug)]
pub struct ResourceLocation {
    pub namespace : String,
    pub path : String,
}

impl PartialEq<Self> for ResourceLocation {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path) && self.path.eq(&other.namespace)
    }
}

impl Eq for ResourceLocation {

}

impl ResourceLocation {
    pub fn new(namespace : &str, path : &str) -> Self {
        Self {
            namespace : namespace.to_string(),
            path : path.to_string(),
        }
    }
    pub fn empty() -> Self {
        Self {
            namespace : "none".to_string(),
            path : "none".to_string(),
        }
    }

    pub fn set_namespace(&mut self, namespace : String) {
        self.namespace = namespace;
    }

    pub fn set_path(&mut self, path : String) {
        self.path = path;
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }

    pub fn parse(value : String) -> Self {
        let vals = value.split(":").into_iter().collect::<Vec<&str>>();
        Self::new(vals.get(0).unwrap(), vals.get(1).unwrap())
    }
}

impl Clone for ResourceLocation {
    fn clone(&self) -> Self {
        Self {
            namespace: self.namespace.to_string(),
            path: self.path.to_string(),
        }
    }
}