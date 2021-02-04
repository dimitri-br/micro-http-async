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
/// 
/// Dynamic variables are defined as `[ var_name ]` in HTML, and
/// just `var_name` in the Vars hashmap
/// 
/// Here is an example:
/// 
/// ```html
/// <p>[ test_var ]</p>
/// ```
/// 
/// ```rust
/// let mut vars = Vars::new();
/// vars.insert("test_var".to_string(), Variable::String("Test".to_string()));
/// ```
/// 
/// This example shows a basic example of how the HTML code matches to Rust.
/// 
/// Please note that if there is no variable defined in the hashmap, it will not update with
/// any dynamic values, and remain static. If the variable in the hashmap doesn't find the variable in the HTML,
/// nothing will happen there as well.
pub struct HtmlConstructor;

impl HtmlConstructor{
    /// # Construct Page
    /// 
    /// Takes in a file path (to the HTML file) and a `Vars` type.
    /// 
    /// Constructs the HTML page, returning a string value (also assigns all dynamic variables if any)
    pub async fn construct_page(path: &str, vars: Vars) -> String{
        let file = FileLoader::load_template(path).await;

        let file = HtmlConstructor::set_dynamic_vars(file, vars);

        return file;
    }

    /// # Set Dynamic Vars
    /// 
    /// Set the dynamic variables in a html file
    /// 
    /// Dynamic variables are defined as `[ var_name ]` in HTML, and
    /// just `var_name` in the Vars hashmap
    fn set_dynamic_vars(mut file: String, vars: Vars) -> String{
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