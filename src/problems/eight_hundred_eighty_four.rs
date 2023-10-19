pub fn run(){
    let equal = backspace_compare(String::from("AN#C"), String::from("do"));
    println!("{}", equal);
}

pub fn simplify_string(st: String) -> String {
    let mut out_st = String::new();

    for i in st.chars() {
        match i {
            '#' => { if !out_st.is_empty() {out_st.pop();}}
            _ => {out_st.push(i)}
        }
    }
    return out_st;
}

pub fn backspace_compare(s: String, t: String) -> bool {
    let _s = simplify_string(s);
    let _t = simplify_string(t);

    return _s == _t;

}