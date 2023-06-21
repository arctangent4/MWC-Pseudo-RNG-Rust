
//MULTIPLY WITH CARRY 32 BIT GENERATOR
pub fn mwc_calc_32_bit(init_x: u32, a: u32, base_val: u32, init_carry_c: u32, lag: u32, num_calcs: u32){
    println!("mwc_32_bit function used.");
    let mut i = 0;
    let mut output_list: Vec<f32>  = Vec::new();
    
    let mut x_next: f32 = init_carry_c as f32;
    let mut carry_c: f32 = init_x as f32;
    let a: f32 = a as f32;

    while i <= lag+num_calcs {
        carry_c = a*x_next+carry_c;
        carry_c = carry_c/(base_val as f32);
        carry_c = carry_c.floor();
        x_next = (a*x_next+carry_c) % (base_val as f32); 

        if i>lag{
            output_list.push(x_next);
        }

        i+=1;
      
    }
    println!("{output_list:?}");

}

//COMPLEMENTARY MWC 32 BIT GENERATOR
pub fn cmwc_calc_32_bit(init_x: u32, a: u32, base_val: u32, init_carry_c: u32, lag: u32, num_calcs: u32){
    println!("cmwc_32_bit function used.");
    let mut i = 0;
    let mut output_list: Vec<f32>  = Vec::new();
    
    let mut x_next: f32 = init_carry_c as f32;
    let mut carry_c: f32 = init_x as f32;
    let a: f32 = a as f32;


    while i <= lag+num_calcs {
        carry_c = a*x_next+carry_c;
        carry_c = carry_c/(base_val as f32);
        carry_c = carry_c.floor();
        x_next = (a*x_next+carry_c) % (base_val as f32); 
        //take the complement for the complementary carry generator
        x_next = (base_val as f32) - x_next; 
        if i>lag{
            output_list.push(x_next);
        }
      

        i+=1;
      
    }
    println!("{output_list:?}");




}