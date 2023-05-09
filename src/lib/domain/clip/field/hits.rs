use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
      self.0
    }
}
