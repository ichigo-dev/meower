//------------------------------------------------------------------------------
//! Traits to load files.
//------------------------------------------------------------------------------

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::Read;


//------------------------------------------------------------------------------
/// Trait to load files into a string map.
//------------------------------------------------------------------------------
pub(crate) trait LoadToStringMap
{
    //--------------------------------------------------------------------------
    /// Loads the i18n map.
    //--------------------------------------------------------------------------
    fn load( &self, path: &Path ) -> HashMap<String, String>
    {
        let path = format!
        (
            "{}/**/*.{{yml,yaml,json,toml,txt,env}}",
            path.to_str().unwrap()
        );

        let glob = globwalk::glob(&path).unwrap();
        let mut map = HashMap::new();
        for entry in glob
        {
            let path = entry.unwrap().into_path();
            if path.is_dir() { continue; }
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap();
            let path = path.to_str().unwrap();
            map.extend(self.load_from_file(path, ext));
        }
        map
    }

    //--------------------------------------------------------------------------
    /// Loads the i18n map.
    //--------------------------------------------------------------------------
    fn load_from_file( &self, path: &str, ext: &str ) -> HashMap<String, String>
    {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        match ext
        {
            "yml" | "yaml" =>
            {
                serde_yaml::from_str(&content).unwrap()
            },
            "json" =>
            {
                serde_json::from_str(&content).unwrap()
            },
            "toml" =>
            {
                toml::from_str(&content).unwrap()
            },
            _ =>
            {
                let mut map = HashMap::new();
                for line in content.lines()
                {
                    if line.contains("=") == false || line.starts_with("#")
                    {
                        continue;
                    }

                    let mut iter = line.splitn(2, "=");
                    let key = iter.next().unwrap().trim().to_string();
                    let value = iter.next().unwrap().trim().to_string();
                    map.insert(key, value);
                }
                map
            },
        }
    }
}
