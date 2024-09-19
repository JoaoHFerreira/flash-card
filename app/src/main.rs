use std::process::Command;
use std::io::stdin;

fn main() {
    
    loop{
        println!("1. For show_posts");
        println!("2. For write_batch_flash_cards");
        println!("3. For write_flash_cards");
        println!("5. For write_subject_learn");
        println!("4. For write_posts");
        println!("6. For Exit");

        let mut str_option = String::new();
        stdin().read_line(&mut str_option).unwrap();
    
        let n_option = str_option.trim_end().parse::<i32>().unwrap();
        
        match n_option{
            1 =>  {
                let output = Command::new("./target/debug/show_posts")
                .output()
                .expect("Failed to execute show_posts");
            println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
            } 
            _ => break,
        }
    }
    
}
