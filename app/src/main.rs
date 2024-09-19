    use std::process::Command;
    use std::io::stdin;

    fn main() {
        
        loop{
            println!("1. For show_posts");
            println!("2. For write_batch_flash_cards");
            println!("3. For write_flash_card");
            println!("4. For write_subject_learn");
            println!("5. For write_posts");
            println!("6. For Exit");
            println!("\n\n");

            let mut str_option = String::new();
            stdin().read_line(&mut str_option).unwrap();
        
            let n_option = str_option.trim_end().parse::<i32>().unwrap();
            
            match n_option{
                1 =>  execute_action("show_posts"),
                2 =>  execute_action("write_batch_flash_cards"),
                3 =>  execute_action("write_flash_card"),
                4 =>  execute_action("write_subject_learn"),
                5 =>  execute_action("write_posts"),
                _ => break,
            }

            println!("\n\n\n\n");
        }
        
    }

    fn execute_action(action: &str) {
        let path = format!("./target/debug/{}", action);
    
        let mut child = Command::new(path)
            .spawn()
            .expect("Failed to execute the action");
        
        child.wait().expect("Failed to wait on child process");
    }