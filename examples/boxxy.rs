#[macro_use] extern crate boxxy;
extern crate tr1pd;
extern crate env_logger;

fn stage1(sh: &mut boxxy::Shell, _args: Vec<String>) -> Result<(), boxxy::Error> {
    shprintln!(sh, "[*] starting stage1");
    tr1pd::sandbox::activate_stage1().unwrap();
    shprintln!(sh, "[+] activated!");
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
