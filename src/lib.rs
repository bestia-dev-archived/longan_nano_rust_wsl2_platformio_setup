// Remove if STD is supported for your platform and you plan to use it
#![no_std]

// Remove if STD is supported for your platform and you plan to use it
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

//
// The functions below are just sample entry points so that there are no linkage errors
// Leave only the one corresponding to your vendor SDK framework
//

////////////////////////////////////////////////////////
// Arduino                                            //
////////////////////////////////////////////////////////

use longan_nano::hal::{pac, prelude::*};
use longan_nano::led::{Led, rgb};

#[no_mangle]
extern "C" fn setup() {
}

#[no_mangle]
#[export_name = "loop"]
extern "C" fn arduino_loop() {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().sysclk(108.mhz()).freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

     let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
     let leds: [&mut dyn Led; 3] = [&mut red, &mut green, &mut blue];

    leds[0].off();
    leds[1].off();
    leds[2].off();

    let mut i = 0;
    let mut on_off = false;
    loop {
        if i > 5_000_000 {
            i = 0;
            if on_off==false{
                leds[2].off();
                on_off=true;                
            }else{
                leds[2].on();
                on_off=false;
            }
        } else{
            i += 1;
        }
    }
}