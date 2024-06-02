use ::std::fs::DirBuilder;

pub struct AppDirStruct {
    pub dirname: String,
    pub path: String,
}

impl AppDirStruct {
    pub fn create() {
        println!("create(): called");

        if cfg!(windows) {
            let user_name = std::env::var("username").expect("NOT FOUND");
            let data_dir = "C:\\Users\\{user_name}\\AppData\\Local\\Ordenee"
                .replace("{user_name}", &user_name);

            match DirBuilder::new().create(data_dir) {
                Ok(()) => {
                    println!("[:ORDENEE:filesystem] Data dir created");
                }
                Err(e) => {
                    println!("[:ORDENEE:filesystem] Failed to create data dir already exists: {e}");
                }
            }
        } else {
            let user_name = std::env::var("USER").expect("NOT FOUND");
            let data_dir = "/home/{user_name}/.Ordenee".replace("{user_name}", &user_name);

            match DirBuilder::new().create(data_dir) {
                Ok(()) => {
                    println!("[:ORDENEE:filesystem] Data dir created")
                }
                Err(e) => {
                    println!("[:ORDENEE:filesystem] Failed to create data dir already exists: {e}");
                }
            }
        }
    }

    pub fn get() -> String {
        if cfg!(windows) {
            let user_name = std::env::var("username").expect("NOT FOUND");
            let data_dir = "C:\\Users\\{user_name}\\AppData\\Local\\Ordenee"
                .replace("{user_name}", &user_name);

            return data_dir;
        } else {
            let user_name = std::env::var("USER").expect("NOT FOUND");
            let data_dir = "/home/{user_name}/.Ordenee".replace("{user_name}", &user_name);

            return data_dir;
        }
    }
}
