use crate::value::{number, sequence, text, tuple};
use std::{collections::BTreeMap, sync::Arc};
use xetal_value::Value;
use xetal_world_surface::{Mesh, from_scene};
#[derive(Clone)]
pub struct Resource {
    pub id: String,
    pub material: String,
    pub revision: u64,
    pub mesh: Arc<Mesh>,
}
#[derive(Default)]
pub struct Cache(pub(crate) BTreeMap<String, Resource>);
impl Cache {
    pub(crate) fn read(&mut self, v: &Value<'_>) -> Result<Vec<Resource>, String> {
        let Value::Tuple(rows) = v else {
            return Err("expected resources tuple".into());
        };
        if rows.len() > 8 {
            return Err("resource budget exceeded".into());
        }
        let mut result = Vec::new();
        for row in rows.iter() {
            let fields = tuple(row, 4)?;
            let id = text(&fields[0])?;
            if result.iter().any(|r: &Resource| r.id == id) {
                return Err("duplicate resource".into());
            }
            let material = text(&fields[1])?;
            if !["ice", "slush", "water"].contains(&material.as_str()) {
                return Err("unsupported material capability".into());
            }
            if result.iter().any(|r| r.material == material) {
                return Err("duplicate material slot".into());
            }
            let revision = sequence(&fields[2])?;
            let mesh = self.mesh(&id, &material, revision, &fields[3])?;
            result.push(Resource {
                id,
                material,
                revision,
                mesh,
            });
        }
        self.0 = result.iter().map(|r| (r.id.clone(), r.clone())).collect();
        Ok(result)
    }
    fn mesh(
        &self,
        id: &str,
        material: &str,
        revision: u64,
        v: &Value<'_>,
    ) -> Result<Arc<Mesh>, String> {
        if let Some(old) = self.0.get(id) {
            if revision < old.revision {
                return Err("resource revision regressed".into());
            }
            if revision == old.revision && material == old.material {
                return Ok(old.mesh.clone());
            }
        }
        let Value::Array(a) = v else {
            return Err("expected triangle array".into());
        };
        if a.data().len() > 864 * 9 {
            return Err("triangle budget exceeded".into());
        }
        let values = a.data().iter().map(number).collect::<Result<Vec<_>, _>>()?;
        Ok(Arc::new(
            from_scene(a.shape(), &values).map_err(|e| e.to_string())?,
        ))
    }
}
