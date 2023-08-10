use std::io;

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

fn display_world(world: Vec<u8>) {
    
    println!("");
    let vector_length = world.len();

    for idx in 0..vector_length {

        if world[idx] != 0 {
            print!("@")
        }
        else { 
            print!(" ")
        }
        //print!("{}", world[idx]);
    }

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
    let mut rule = String::new();
    let mut steps = String::new(); 
    let mut world_size = String::new();

    //let halfway_idx = world_size / 2;
    //let zero_vec = vec![0; len];
    //let mut world: Vec<u8> = vec![0; world_size];
    //world[halfway_idx] = 1;
    //let mut world:  [u8; 11] = [0,0,0,0,0,1,0,0,0,0,0];

    println!("How big do you want the 1D CA world to be?");

    io::stdin()
        .read_line(&mut world_size)
        .expect("Failed!!!");

    let world_size: u8 = world_size.trim().parse().unwrap();
    let halfway_idx: usize = (world_size / 2).into();
    println!("CA world will be {world_size} elements, {halfway_idx}");

    let mut world: Vec<u8> = vec![0; world_size.into()];
    world[halfway_idx] = 1;

    println!("Enter the elementary CA rule. e.g. 110 corresponds to 0b0011_1110");

    io::stdin()
        .read_line(&mut rule)
        .expect("Failed!!!");

    println!("You entered: {}", rule.trim());

    let rule: u8 = rule.trim().parse().unwrap();
    let rule_array = rule_to_array(rule);

    display_rule_array(rule_array);
    println!("   In binary: {:08.b}", rule);

    println!("Enter the number of steps to compute: ");

    io::stdin()
        .read_line(&mut steps)
        .expect("Failed!!!");

    let steps: usize = steps.trim().parse().unwrap();

    
    for my_step in 0..steps {

        display_world(world.clone());
        world = update_world(world.clone(), rule_array);
    }
    display_world(world.clone());
    println!("");


}
