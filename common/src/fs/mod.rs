use ::std::fs::DirBuilder;
use ::std::os;
use std::process::Command;

pub struct AppDirStruct {
    pub dirname: String,
    pub path: String,
}

impl AppDirStruct {
    pub fn build_app_dir() {
        println!("build_app_dir(): called");

        if cfg!(windows) {
            let child_result = Command::new("echo %USERNAME%")
                .output()
                .expect("[:ORDENEE:] Failed to get current user");

            let user_name = String::from_utf8_lossy(&child_result.stdout);
            let user_name = String::from(user_name);
            let data_dir = "C:\\Users\\{user_name}\\AppData\\Local\\Ordenee".replace("{user_name}", &user_name);

            DirBuilder::new().create(data_dir).expect("[:ORDENEE:] Failed to create data dir");
        } else {
            let user_name = std::env::var("USER").expect("NOT FOUND");
            let data_dir = "/home/{user_name}/.Ordenee".replace("{user_name}", &user_name);
            println!("Data dir: {data_dir}");


            DirBuilder::new().create(data_dir).expect("[:ORDENEE:] Failed to create data dir");
        }    
    }
}
