extern crate boxxy;
extern crate tr1pd;
extern crate env_logger;

fn stage1(_args: Vec<String>) -> Result<(), boxxy::Error> {
    println!("[*] starting stage1");
    tr1pd::sandbox::activate_stage1().unwrap();
    println!("[+] activated!");
    Ok(())
}

fn main() {
    env_logger::init();

    println!("stage1        activate sandbox stage1/1");

    let toolbox = boxxy::Toolbox::new().with(vec![
            ("stage1", stage1),
        ]);
    boxxy::Shell::new(toolbox).run()
}
