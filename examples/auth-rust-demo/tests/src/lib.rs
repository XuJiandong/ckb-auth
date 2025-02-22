use ckb_testtool::ckb_types::bytes::Bytes;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg(test)]
mod tests;

const TEST_ENV_VAR: &str = "CAPSULE_TEST_ENV";

pub enum TestEnv {
    Debug,
    Release,
}

impl FromStr for TestEnv {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(TestEnv::Debug),
            "release" => Ok(TestEnv::Release),
            _ => Err("no match"),
        }
    }
}

pub struct Loader(PathBuf);

impl Default for Loader {
    fn default() -> Self {
        let test_env = match env::var(TEST_ENV_VAR) {
            Ok(val) => val.parse().expect("test env"),
            Err(_) => TestEnv::Debug,
        };
        Self::with_test_env(test_env)
    }
}

impl Loader {
    fn with_test_env(env: TestEnv) -> Self {
        let load_prefix = match env {
            TestEnv::Debug => "debug",
            TestEnv::Release => "release",
        };
        let _dir = env::current_dir().unwrap();
        let mut base_path = PathBuf::new();
        // cargo may use a different cwd when running tests, for example:
        // when running debug in vscode, it will use workspace root as cwd by default,
        // when running test by `cargo test`, it will use tests directory as cwd,
        // so we need a fallback path
        base_path.push("build");
        if !base_path.exists() {
            base_path.pop();
            base_path.push("..");
            base_path.push("build");
        }
        base_path.push(load_prefix);
        Loader(base_path)
    }

    fn load_binary(&self, name: &str) -> Bytes {
        let mut path = self.0.clone();
        path.push(name);
        fs::read(path).expect("binary").into()
    }

    pub fn load_demo(&self) -> Bytes {
        self.load_binary("auth-rust-demo")
    }
    pub fn load_auth(&self) -> Bytes {
        self.load_binary("../../../../build/auth")
    }
    pub fn load_secp256k1_data(&self) -> Bytes {
        self.load_binary("../../../../build/secp256k1_data_20210801")
    }
}
