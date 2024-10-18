mod config;
mod depviz;

fn main() {
    let config = config::load_config("src/cfg/cfg.xml");

    let depviz = depviz::Depviz::new(&config.binpath, &config.outputpath);
    
    // Создать enum для output_format
    match depviz.visualize(&config.package, config.maxdepth, "svg") {
        Ok(_) => {},
        Err(e) => panic!("panic while visualizing dependencies: {}", e)
    }
}
