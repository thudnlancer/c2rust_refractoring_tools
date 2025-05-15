use std::process;

fn xalloc_die() -> ! {
    let msg = gettextrs::gettext("memory exhausted");
    eprintln!("{}", msg);
    process::exit(1);
}