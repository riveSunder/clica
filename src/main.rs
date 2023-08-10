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

fn update_world(world: [u8; 11], rule_array: [bool; 8]) -> [u8; 11] {

    let w: [u8; 11] = world;
    let r: [bool; 8] = rule_array;

    let mut new_world: [u8; 11] = [0,0,0,0,0,0,0,0,0,0,0];
    let mut l_idx: usize = 0;
    let mut r_idx: usize = 0;

    let indices: [usize; 11] = [0,1,2,3,4,5,6,7,8,9,10];
    for idx in indices {


        // identify the edges
        if idx == 0 {
            l_idx = 10;
        }
        else {
            l_idx = idx - 1;
        }
        if idx == 10 {
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

fn display_world(world: [u8; 11]) {
    
    println!("");

    for idx in 0..11 {

        print!("{}", world[idx]);
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


    //let world_size: usize = 11;
    //let halfway_idx = world_size / 2;
    //let zero_vec = vec![0; len];
    //let mut world: Vec<u8> = vec![0; world_size];
    //world[halfway_idx] = 1;
    let mut world: [u8; 11] = [0,0,0,0,0,1,0,0,0,0,0];

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

        display_world(world);
        world = update_world(world, rule_array);

    }
    display_world(world);
    println!("");

}
