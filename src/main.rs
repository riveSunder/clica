use std::io;
use std::env;


fn rule_to_array(rule: u8) -> [bool; 8]{
    // converts a decimal rule to a tuple of booleans

    let mut rule_array: [bool; 8] = [
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false];

    let base: u8 = 2;
    let mut rule = rule;

    for idx in 0..8 {

        //println!("{}", base.pow(my_idx));
        let my_power: u8 = 7 - idx;

        if rule >= base.pow(my_power.into()){

            let my_idx: usize = idx.into();
            rule_array[my_idx] = true; 

            rule = rule - base.pow(my_power.into());
        }
        
    }

    return rule_array
}

fn update_world(world: Vec<u8>, rule_array: [bool; 8]) -> Vec<u8> {

    let r: [bool; 8] = rule_array;

    let vector_length = world.len();
    let mut new_world: Vec<u8> = vec![0; vector_length];
    //let w: vec![0, vector_length.into()] = world;
    let w = world;

    let mut l_idx: usize = 0;
    let mut r_idx: usize = 0;

    for my_idx in 0..vector_length {

        let idx: usize = my_idx.into();

        // identify the edges
        if idx == 0 {
            l_idx = vector_length - 1;
        }
        else {
            l_idx = idx - 1;
        }
        if idx == vector_length-1 {
            r_idx = 0;
        }
        else {
            r_idx = idx + 1;
        }
        
        
        if (w[l_idx] != 0) 
                && (w[idx] != 0)
                && (w[r_idx] != 0)
                && (r[0]) {
            new_world[idx] = 1;
        }
        else if (w[l_idx] != 0) 
                && (w[idx] != 0)
                && (w[r_idx] == 0)
                && (r[1]) {
            new_world[idx] = 1;
        }
        else if (w[l_idx] != 0) 
                && (w[idx] == 0)
                && (w[r_idx] != 0)
                && (r[2]) {
            new_world[idx] = 1;
        }
        else if (w[l_idx] != 0) 
                && (w[idx] == 0)
                && (w[r_idx] == 0)
                && (r[3]) {
            new_world[idx] = 1;
        }
        else if (w[l_idx] == 0) 
                && (w[idx] != 0)
                && (w[r_idx] != 0)
                && (r[4]) {
            new_world[idx] = 1;
        }
        else if (w[l_idx] == 0) 
                && (w[idx] != 0)
                && (w[r_idx] == 0)
                && (r[5]) {
            new_world[idx] = 1;
        }
        else if (w[l_idx] == 0) 
                && (w[idx] == 0)
                && (w[r_idx] != 0)
                && (r[6]) {
            new_world[idx] = 1;
        }
        else if (w[l_idx] == 0) 
                && (w[idx] == 0)
                && (w[r_idx] == 0)
                && (r[7]) {
            new_world[idx] = 1;
        }

    }
        
    return new_world
}

fn display_world(world: Vec<u8>) -> String {
    
    let mut my_string_world: String = "".to_string();

    let vector_length = world.len();

    for idx in 0..vector_length {

        if world[idx] != 0 {
            my_string_world = my_string_world + "@"
        }
        else { 
            my_string_world = my_string_world + " "
        }
        //print!("{}", world[idx]);
    }
    my_string_world = my_string_world + "\n";

    return my_string_world
}

fn display_rule_array(rule_array: [bool; 8]) {

    println!("rule as boolean array: ");
    print!("        ");

    for idx in 0..8 {

        if rule_array[idx] {
            print!("1");
        }
        else {
            print!("0");
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let number_args: usize = args.len();

    
    let mut rule: u8 = 1;
    let mut steps: usize = 1; 
    let mut world_size: usize = 1;
    let mut output_file: String = "".to_string();

    let mut rule_set: bool = false;
    let mut steps_set: bool = false; 
    let mut world_size_set: bool = false;
    let mut output_file_set: bool = false;

    for ii in 1..number_args {
        let element = &args[ii];

        if ii < (number_args+1) { 
            if element == "-d" {
                let next_element = &args[ii+1];
                world_size =  next_element.trim().parse().unwrap();
                world_size_set = true;
                println!("{next_element} {world_size}");
            }
            if element == "-s" {
                let next_element = &args[ii+1];
                steps =  next_element.trim().parse().unwrap();
                steps_set = true;
                println!("{next_element} {steps}");
            }
            if element == "-r" {
                let next_element = &args[ii+1];
                rule =  next_element.trim().parse().unwrap();
                rule_set = true;
                println!("{next_element} {rule}");
            }
            if element == "-o" {
                let next_element = &args[ii+1];
                output_file =  next_element.trim().parse().unwrap();
                output_file_set = true;
                println!("{next_element} {output_file}");
            }

        }

    }

    if !(rule_set) {
        // if not supplied as cli args, ask the user for rule number

        let mut user_rule = String::new();

        println!("Enter the elementary CA rule. e.g. 110 corresponds to 0b0011_1110");

        io::stdin()
            .read_line(&mut user_rule)
            .expect("Failed!!!");

        println!("You entered: {}", user_rule.trim());

        rule = user_rule.trim().parse().unwrap();
    }

    if !(world_size_set) {


        let mut user_world_size = String::new();
        println!("How big do you want the 1D CA world to be?");
        io::stdin()
            .read_line(&mut user_world_size)
            .expect("Failed!!!");
        world_size = user_world_size.trim().parse().unwrap();
        println!("CA world will be {world_size} elements");
    }

    if !(steps_set) {
        let mut user_steps = String::new(); 
        println!("Enter the number of steps to compute: ");

        io::stdin()
            .read_line(&mut user_steps)
            .expect("Failed!!!");

        steps = user_steps.trim().parse().unwrap();

    }

    let rule_array = rule_to_array(rule);
    let mut world: Vec<u8> = vec![0; world_size.into()];
    let halfway_idx: usize = (world_size / 2).into();
    world[halfway_idx] = 1;

    display_rule_array(rule_array);
    println!("   In binary: {:08.b}", rule);


    
    for my_step in 0..steps {

        let my_line = display_world(world.clone());
        print!("{}", my_line);
        world = update_world(world.clone(), rule_array);
    }
    display_world(world.clone());
    println!("");


}
