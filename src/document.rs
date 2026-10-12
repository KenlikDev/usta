//! Versioned, renderer-independent scene document and local JSON persistence.
//!
//! This module intentionally does not depend on Bevy or any GPU/UI types.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

pub const CURRENT_SCHEMA_VERSION: u32 = 1;
pub const MAX_SCENE_OBJECTS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrimitiveKind {
    Cube,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneObject {
    /// Stable document identifier. This is not a Bevy entity ID.
    pub id: String,
    pub name: String,
    pub kind: PrimitiveKind,
    pub translation: [f32; 3],
    /// Quaternion components in [x, y, z, w] order.
    pub rotation_xyzw: [f32; 4],
    pub scale: [f32; 3],
}

impl SceneObject {
    pub fn cube(id: impl Into<String>, name: impl Into<String>, translation: [f32; 3]) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind: PrimitiveKind::Cube,
            translation,
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneDocument {
    pub schema_version: u32,
    pub objects: Vec<SceneObject>,
}

impl SceneDocument {
    pub fn new(objects: Vec<SceneObject>) -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            objects,
        }
    }

    /// Reject unsupported or unsafe documents before changing the live scene.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported scene schema version {}; this build supports version {}",
                self.schema_version, CURRENT_SCHEMA_VERSION
            ));
        }

        if self.objects.is_empty() {
            return Err("scene contains no editable objects".to_owned());
        }

        if self.objects.len() > MAX_SCENE_OBJECTS {
            return Err(format!(
                "scene contains {} objects; the limit is {}",
                self.objects.len(),
                MAX_SCENE_OBJECTS
            ));
        }

        let mut ids = HashSet::with_capacity(self.objects.len());
        for object in &self.objects {
            if object.id.trim().is_empty() {
                return Err("object IDs must not be empty".to_owned());
            }
            if object.name.trim().is_empty() {
                return Err(format!("object {} has an empty name", object.id));
            }
            if !ids.insert(object.id.as_str()) {
                return Err(format!("duplicate object ID: {}", object.id));
            }

            if object
                .translation
                .iter()
                .chain(object.rotation_xyzw.iter())
                .chain(object.scale.iter())
                .any(|value| !value.is_finite())
            {
                return Err(format!(
                    "object {} has a non-finite transform value",
                    object.id
                ));
            }

            let rotation_length_squared = object
                .rotation_xyzw
                .iter()
                .map(|value| value * value)
                .sum::<f32>();
            if rotation_length_squared < 1.0e-12 {
                return Err(format!(
                    "object {} has an invalid zero-length rotation quaternion",
                    object.id
                ));
            }

            if object.scale.iter().any(|value| value.abs() < 1.0e-4) {
                return Err(format!(
                    "object {} has a scale component too close to zero",
                    object.id
                ));
            }
        }

        Ok(())
    }

    pub fn to_json_pretty(&self) -> Result<String, String> {
        self.validate()?;
        serde_json::to_string_pretty(self)
            .map_err(|error| format!("could not serialize scene: {error}"))
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let document: Self =
            serde_json::from_str(json).map_err(|error| format!("invalid scene JSON: {error}"))?;
        document.validate()?;
        Ok(document)
    }

    /// Atomically replace the destination after a full serialization and write.
    pub fn save_to(&self, path: &Path) -> Result<(), String> {
        let bytes = self.to_json_pretty()?;
        let mut temporary_name = path.as_os_str().to_os_string();
        temporary_name.push(".tmp");
        let temporary_path = PathBuf::from(temporary_name);

        let result = (|| {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&temporary_path)
                .map_err(|error| {
                    format!(
                        "cannot create temporary scene file {}: {error}",
                        temporary_path.display()
                    )
                })?;
            file.write_all(bytes.as_bytes()).map_err(|error| {
                format!(
                    "cannot write temporary scene file {}: {error}",
                    temporary_path.display()
                )
            })?;
            file.sync_all().map_err(|error| {
                format!(
                    "cannot flush temporary scene file {}: {error}",
                    temporary_path.display()
                )
            })?;
            drop(file);

            fs::rename(&temporary_path, path).map_err(|error| {
                format!("cannot replace scene file {}: {error}", path.display())
            })?;
            Ok(())
        })();

        if result.is_err() {
            let _ = fs::remove_file(&temporary_path);
        }
        result
    }

    pub fn load_from(path: &Path) -> Result<Self, String> {
        let json = fs::read_to_string(path)
            .map_err(|error| format!("cannot read scene file {}: {error}", path.display()))?;
        Self::from_json(&json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn sample_document() -> SceneDocument {
        let mut cube = SceneObject::cube("cube-0001", "Cube", [1.0, 2.0, 3.0]);
        cube.rotation_xyzw = [0.0, 0.0, 0.38268343, 0.9238795];
        cube.scale = [1.0, 2.0, 1.5];
        SceneDocument::new(vec![cube])
    }

    fn temporary_path(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after UNIX epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "usta-scene-document-{}-{label}-{nonce}.json",
            std::process::id()
        ))
    }

    #[test]
    fn json_round_trip_preserves_object_identity_and_transform() {
        let original = sample_document();
        let json = original
            .to_json_pretty()
            .expect("document should serialize");
        let loaded = SceneDocument::from_json(&json).expect("document should deserialize");
        assert_eq!(loaded, original);
    }

    #[test]
    fn file_round_trip_preserves_scene_document() {
        let path = temporary_path("round-trip");
        let original = sample_document();

        original.save_to(&path).expect("scene should save");
        let loaded = SceneDocument::load_from(&path).expect("scene should load");

        assert_eq!(loaded, original);
        fs::remove_file(path).expect("temporary scene file should be removed");
    }

    #[test]
    fn unsupported_schema_version_is_rejected() {
        let json = r#"{"schema_version":99,"objects":[{"id":"cube-0001","name":"Cube","kind":"cube","translation":[0.0,0.5,0.0],"rotation_xyzw":[0.0,0.0,0.0,1.0],"scale":[1.0,1.0,1.0]}]}"#;
        let error = SceneDocument::from_json(json).expect_err("unsupported version must fail");
        assert!(error.contains("unsupported scene schema version"));
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let cube = SceneObject::cube("same-id", "Cube", [0.0, 0.5, 0.0]);
        let document = SceneDocument::new(vec![cube.clone(), cube]);
        let error = document.validate().expect_err("duplicate IDs must fail");
        assert!(error.contains("duplicate object ID"));
    }

    #[test]
    fn non_finite_transform_values_are_rejected_before_writing() {
        let mut cube = SceneObject::cube("cube-0001", "Cube", [0.0, 0.5, 0.0]);
        cube.translation[0] = f32::NAN;
        let document = SceneDocument::new(vec![cube]);
        assert!(document.validate().unwrap_err().contains("non-finite"));
    }

    #[test]
    fn zero_quaternion_is_rejected() {
        let mut cube = SceneObject::cube("cube-0001", "Cube", [0.0, 0.5, 0.0]);
        cube.rotation_xyzw = [0.0; 4];
        let document = SceneDocument::new(vec![cube]);
        assert!(document.validate().unwrap_err().contains("zero-length"));
    }

    #[test]
    fn malformed_json_is_rejected() {
        assert!(SceneDocument::from_json("{not json").is_err());
    }

    #[test]
    fn empty_scene_is_rejected_for_this_prototype() {
        let document = SceneDocument::new(Vec::new());
        assert!(
            document
                .validate()
                .unwrap_err()
                .contains("no editable objects")
        );
    }
}
