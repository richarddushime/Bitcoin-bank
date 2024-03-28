use directories::UserDirs;
use std::{
    fs::DirBuilder,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone)]
pub struct DataDirs {
    pub app_dir: PathBuf,
    pub mining: PathBuf,
    pub cold: PathBuf,
    pub hot: PathBuf,
}

impl DataDirs {
    pub fn get_dirs() -> Self {
        let app_dir = DataDirs::app_dir();
        let mining = DataDirs::mining_node_dir(&app_dir);
        let cold = DataDirs::cold_node_dir(&app_dir);
        let hot = DataDirs::hot_node_dir(&app_dir);

        Self {
            app_dir,
            mining,
            cold,
            hot,
        }
    }
    pub fn app_dir() -> PathBuf {
        let mut app_dir = UserDirs::new().unwrap().home_dir().to_path_buf();
        app_dir.push("bitcoin-bank-data");
        DirBuilder::new().recursive(true).create(&app_dir).unwrap();

        app_dir
    }

    pub fn mining_node_dir(app_dir_path: &Path) -> PathBuf {
        DataDirs::node_dir(app_dir_path, "mining-node")
    }
    pub fn cold_node_dir(app_dir_path: &Path) -> PathBuf {
        DataDirs::node_dir(app_dir_path, "cold-node")
    }

    pub fn hot_node_dir(app_dir_path: &Path) -> PathBuf {
        DataDirs::node_dir(app_dir_path, "hot-node")
    }

    pub fn node_dir(app_dir_path: &Path, dir_name: &str) -> PathBuf {
        let mut node_path = app_dir_path.to_path_buf();
        node_path.push(dir_name);
        DirBuilder::new()
            .recursive(true)
            .create(&node_path)
            .unwrap();

        node_path
    }
}
