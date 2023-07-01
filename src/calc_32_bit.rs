
//MULTIPLY WITH CARRY 32 BIT GENERATOR

pub fn mwc_calc_32_bit(init_x: u32, a: u32, base_val: u32, init_carry_c: u32, lag: u32, num_calcs: u32) {
    println!("mwc_32_bit function used.\n");
    // Converting types and Initialization
    //Generator Parameters
    let mut x_next: u32 = init_carry_c as u32;
    let mut carry_c: u32 = init_x; 
    let a: u32 = a as u32;

    //Utility Variables
    let mut i = 0;
    let mut output_list: Vec<u32> = Vec::new();
    let mut store_vals: Vec<u32> = Vec::new();

    let mut interm_result: u32;
    let mut lagged_index: u32;
    

    // Pregenerating the lagging sequence and then returning desired number of 'random' numbers
    while i <= lag + num_calcs {
        // The carrying constant c is always the first thing to be manipulated.
        // Note the first instantiation of x_next is not an actual term in the sequence, it is just the seed of the generator.
        
        interm_result = a.wrapping_mul(x_next); 
        interm_result = interm_result.wrapping_add(carry_c);
        carry_c = ((interm_result / (base_val))) as u32;


        // After c is found, a linear expression is evaluated and then mod base_val is taken(remember base_val = b-1).
        // Similar to a Lehmer random number generator
        interm_result= a.wrapping_mul(x_next);
        interm_result = interm_result.wrapping_add(carry_c);
        x_next = interm_result % (base_val);

        //Logic to properly "lag" the generator.  store_vals is filled with lagged values during the pregeneration step.
        store_vals.push(x_next);

        if i == lag-1 {
            x_next = store_vals[0];

        } else if i >= lag{
             //Once lagging is finished, the lagged values will be used to generate the output sequence.
            output_list.push(x_next);
            lagged_index = i+1 - lag;
            x_next = store_vals[lagged_index as usize];
        }

        i += 1;
    }

    println!("{output_list:?}");
}

//COMPLEMENTARY MWC 32 BIT GENERATOR

pub fn cmwc_calc_32_bit(init_x: u32, a: u32, base_val: u32, init_carry_c: u32, lag: u32, num_calcs: u32) {
    println!("cmwc_32_bit function used.\n");
    // Converting types and Initialization
    //Generator Parameters
    let mut x_next: u32 = init_carry_c as u32;
    let mut carry_c: u32 = init_x; 
    let a: u32 = a as u32;

    //Utility Variables
    let mut i = 0;
    let mut output_list: Vec<u32> = Vec::new();
    let mut store_vals: Vec<u32> = Vec::new();

    let mut interm_result: u32;
    let mut lagged_index: u32;
    

    // Pregenerating the lagging sequence and then returning desired number of 'random' numbers
    while i <= lag + num_calcs {
        // The carrying constant c is always the first thing to be manipulated.
        // Note the first instantiation of x_next is not an actual term in the sequence, it is just the seed of the generator.
        
        interm_result = a.wrapping_mul(x_next); 
        interm_result = interm_result.wrapping_add(carry_c);
        carry_c = ((interm_result / (base_val))) as u32;


        //The main difference between CMWC AND MWC: we take the complement of x_next relative to base_val
        interm_result= a.wrapping_mul(x_next);
        interm_result = interm_result.wrapping_add(carry_c);
        interm_result = interm_result % (base_val);
        x_next = base_val.wrapping_sub(interm_result);
        

       //Logic to properly "lag" the generator.  store_vals is filled with lagged values during the pregeneration step.
        store_vals.push(x_next);

        if i == lag-1 {
            x_next = store_vals[0];

        } else if i >= lag{
            //Once lagging is finished, the lagged values will be used to generate the output sequence.
            output_list.push(x_next);
            lagged_index = i+1 - lag;
            x_next = store_vals[lagged_index as usize];
        }

        i += 1;
    }

    println!("{output_list:?}");
}