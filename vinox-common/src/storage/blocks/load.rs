use directories::ProjectDirs;
use std::fs::{self};

use walkdir::WalkDir;

use super::descriptor::BlockDescriptor;

pub fn load_all_blocks() -> Vec<BlockDescriptor> {
    let mut result = Vec::new();
    if let Some(proj_dirs) = ProjectDirs::from("com", "vinox", "vinox") {
        for entry in WalkDir::new(proj_dirs.data_dir().join("assets"))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().extension().unwrap_or_default() == "ron" {
                if let Ok(ron_string) = fs::read_to_string(entry.path()) {
                    if let Ok(block) = ron::from_str(ron_string.as_str()) {
                        result.push(block);
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::storage::blocks::descriptor::{BlockDescriptor, ToolType};

    #[test]
    fn it_works() {
        let ron_type = "
            BlockDescriptor(
                namespace: \"vinox\",
                name: \"grass\",
                tool_type: Some(Shovel)
            )
        ";
        if let Ok(ron_string) = ron::from_str::<BlockDescriptor>(ron_type) {
            assert_eq!(
                ron_string,
                BlockDescriptor {
                    namespace: "vinox".to_string(),
                    name: "grass".to_string(),
                    tool_type: Some(ToolType::Shovel),
                    ..Default::default()
                }
            )
        }
    }
}
