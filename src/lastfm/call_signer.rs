use std::collections::HashMap;

pub struct CallSigner<'a> {
    shared_secret: &'a str,
}

impl<'a> CallSigner<'a> {
    pub fn new(shared_secret: &'a str) -> Self {
        Self { shared_secret }
    }

    pub fn sign(&self, query_params: &HashMap<String, String>) -> String {
        let mut sorted_keys = query_params.keys().collect::<Vec<&String>>();
        sorted_keys.sort();

        let mut result: Vec<String> = Vec::new();

        for key in sorted_keys {
            result.push(format!("{}{}", key, query_params.get(key).unwrap()));
        }

        let digest = md5::compute(format!("{}{}", result.join(""), self.shared_secret));

        format!("{:x}", digest)
    }
}
