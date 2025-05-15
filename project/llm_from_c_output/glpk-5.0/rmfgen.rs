/* rmfgen.rs */

use std::process;

pub fn glp_rmfgen(
    G_: &glp_graph,
    s_: &i32,
    t_: &i32,
    a_cap_: i32,
    parm: &[i32; 6],
) -> Result<i32, String> {
    const FUNC: &str = "glp_rmfgen";
    
    // Assertions equivalent to the C version's xassert macros
    assert!(std::ptr::eq(G_, G_));
    assert!(std::ptr::eq(s_, s_));
    assert!(std::ptr::eq(t_, t_));
    assert_eq!(a_cap_, a_cap_);
    assert!(std::ptr::eq(parm, parm));

    let error_msg = format!(
        "{}: sorry, this routine is temporarily disabled due to licensing problems",
        FUNC
    );
    eprintln!("{}", error_msg);
    process::abort();
    
    // This line is unreachable due to abort(), but included to match C return
    Err(error_msg)
}

/* eof */