mod calc_32_bit;
mod calc_64_bit;


use std::process::ExitCode;
use rand::random;

use calc_32_bit::mwc_calc_32_bit;
use calc_32_bit::cmwc_calc_32_bit;

use calc_64_bit::mwc_calc_64_bit;
use calc_64_bit::cmwc_calc_64_bit;

//See mwc_calc_64_bit function for explanation of the process
fn main() {
    //These constants are needed to initialize the program. 
    //Capitalization for constants, lowercase for variables
    
    const LAGGING_CONSTANT: u32 = 5;
    const NUM_CALCS: u32 = 3;


    const INIT_C: u32 = 809430660;
  

    

    // println!(" Welcome to the multiply-with-carry(MWC) pseudorandom generator. Please input the rng seed you want ");
    // io::stdin().read_line(&mut rng_seed).expect("failed to readline");

    let cmwc: bool = false;
    let bit_size: u32;
    let base_val: u64;
    let a: u32 = 18782;

    
    #[cfg(target_pointer_width = "32")]
    {
        //init_x is chosen to be randomly selected, just like in Marsaglia's implementation.
        //However, it can be manually changed with something like let init_x: u32 = 3.
        let rand_x: u32; 
        init_x = random();
        println!("The randomly generated seed for the generator is {}", init_x);

        bit_size = 32;
        base_val = u32::max_value() as u64 - 1;

        if cmwc {
            cmwc_calc_32_bit(init_x,a, base_val, INIT_C, LAGGING_CONSTANT, NUM_CALCS);
        } else {
            mwc_calc_32_bit(init_x,a, base_val, INIT_C, LAGGING_CONSTANT, NUM_CALCS);
        }
    }

    #[cfg(target_pointer_width = "64")]
    {

        let init_x: u64; 
        init_x = random();
        println!("The randomly generated seed for the generator is {}", init_x);

        bit_size = 64;
        base_val = u64::max_value() - 1;

        if cmwc {
            cmwc_calc_64_bit(init_x,a, base_val, INIT_C, LAGGING_CONSTANT, NUM_CALCS);
        } else {
            mwc_calc_64_bit(init_x,a, base_val, INIT_C, LAGGING_CONSTANT, NUM_CALCS);
        }
    }

    
    if !(bit_size == 64 || bit_size==32)  {
        println!("Your operating system's bitsize is 
        not supported.");
        ExitCode::FAILURE;
        return;
    } 

}