// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {  
    let mut res: i32 = 42; // 确保使用足够大的数据类型  
    let option = Some(12);  
    if let Some(x) = option {  
        res += x; // 这里不会发生溢出  
    }  
    println!("{}", res);  
}
