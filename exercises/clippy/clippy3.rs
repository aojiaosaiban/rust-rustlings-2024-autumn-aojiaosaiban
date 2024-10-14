// clippy3.rs  

fn main() {  
    let my_option: Option<()> = None;  

    // 没有必要在这里调用 `.unwrap()`，因为 `my_option.is_none()` 是 true  
    // 直接忽略这个分支，或者根据需要处理它。  
    if my_option.is_none() {  
        // 此处可以直接处理而不是 unwrap()  
    }  

    let my_arr = &[  
        -1, -2, -3,  
        -4, -5, -6,  
    ];  
    println!("My array! Here it is: {:?}", my_arr);  

    // 使用 Vec::new() 创建一个空的 Vec，而不是 resize()  
    let my_empty_vec: Vec<i32> = Vec::new();  
    println!("This Vec is empty, see? {:?}", my_empty_vec);   

    // 交换两个值  
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a,&mut value_b );
    println!("value a: {}; value b: {}", value_a, value_b); 
}