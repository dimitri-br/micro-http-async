use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::fs::File;


/// # File Loader
/// 
/// Loads a html file from a templates folder. Its sole purpose it to load files asynchrynously
pub struct FileLoader;

impl FileLoader{
    pub async fn load_template(path: &str) -> String{
        let mut file_handle = File::open(path).await.unwrap();
        
        let mut contents = vec![];
        file_handle.read_to_end(&mut contents).await.unwrap();
        
        String::from_utf8(contents).expect("File not valid UTF-8")
    }
}


/// Enum to wrap variables for passing into the HtmlConstructor when constructing
/// a html page
pub enum Variable{
    Int(i32),
    Float(f32),
    UInt(usize),
    String(String),
}

/// Holds all variables to dynamically generate (similar to jinja in python)
pub type Vars = HashMap<String, Variable>;

/// # Html Constructor
/// 
/// Loads a html file and constructs it for dynamic web loading
/// 
/// For example, when loading a template, you can pass through variables in a vec to implement them in your page
/// 
/// You can use this for things like user info or other server side information
/// 
/// Alternatively you could create a static page with JavaScript that loads this information from a webapi route
/// on the same server. 
pub struct HtmlConstructor;

impl HtmlConstructor{
    pub async fn construct_page(path: &str, vars: Vars) -> String{
        let file = FileLoader::load_template(path).await;

        let file = HtmlConstructor::set_dynamic_vars(file, vars);

        return file;
    }

    pub fn set_dynamic_vars(mut file: String, vars: Vars) -> String{
        for (key, var) in vars.iter(){
            let var_to_replace = format!("[ {} ]", key);
            match var{
                Variable::Int(v) => {
                    file = file.replace(&var_to_replace, &v.to_string());
                },
                Variable::Float(v) => {
                    file = file.replace(&var_to_replace, &v.to_string());
                },
                Variable::UInt(v) => {
                    file = file.replace(&var_to_replace, &v.to_string());
                },
                Variable::String(v) => {
                    file = file.replace(&var_to_replace, &v);
                },
            };
        }

        file
    }
}