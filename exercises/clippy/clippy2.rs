// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


#[allow(for_loops_over_fallibles)]/*编译器建议的，这是啥
又看了一下，也有建议改成if let. 这个格式确实适合枚举类型
*/
fn main() {
    let mut res = 42;
    let option = Some(12);
    //if let Some(x)=option{res+=x}
    for x in option {
        res += x;
    }
    println!("{}", res);
}
