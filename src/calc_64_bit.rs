//MULTIPLY WITH CARRY 64 BIT GENERATOR
pub fn mwc_calc_64_bit(init_x: u64, a: u32, base_val: u64, init_carry_c: u32, lag: u32, num_calcs: u32){
    println!("mwc_64_bit function used.");
    //Converting types
    let mut i = 0;
    let mut output_list: Vec<f64>  = Vec::new();
    
    let mut x_next: f64 = init_carry_c as f64;
    let mut carry_c: f64 = init_x as f64;
    let a: f64 = a as f64;

    //Pregenerating the lagging sequence and then returning desired number of 'random' numbers
    while i <= lag+num_calcs {
       
        //The carrying constant c is the first thing to always be manipulated. Note the first instantiation of x_next
        //is not an actual term in the sequence, it is just the seed of the generator.
        carry_c = a*x_next+carry_c;
        carry_c = carry_c/(base_val as f64);
        carry_c = carry_c.floor();

        //After c is found, a linear expression is evaluated and then mod b-1 is taken. 
        //Similar to a Lehmer random number generator 
        x_next = (a*x_next+carry_c) % (base_val as f64); 


        if i>lag{
            output_list.push(x_next);
        }

        i+=1;
      
    }
    println!("{output_list:?}");
}

//COMPLEMENTARY MWC 64 BIT GENERATOR
pub fn cmwc_calc_64_bit(init_x: u64, a: u32, base_val: u64, init_carry_c: u32, lag: u32, num_calcs: u32) {
    println!("cmwc_64_bit function used.");
    let mut i = 0;
    let mut output_list: Vec<f64>  = Vec::new();
    
    let mut x_next: f64 = init_carry_c as f64;
    let mut carry_c: f64 = init_x as f64;
    let a: f64 = a as f64;

    while i <= lag+num_calcs {
       
        carry_c = a*x_next+carry_c;
        carry_c = carry_c/(base_val as f64);
        carry_c = carry_c.floor();
        x_next = (a*x_next+carry_c) % (base_val as f64); 
        //The complementary generator is formed by taking x_next and subtracting it from b-1.
        x_next = (base_val as f64) - x_next; 



        if i>lag{
            output_list.push(x_next);
        }

        i+=1;
      
    }
    println!("{output_list:?}");



}