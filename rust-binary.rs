use std::io;

fn main()  {
    // seting up nums
    let mut base_str = String::new();
    let mut count = 0;
    let mut num_str = String::new();

    // taking input for the base
    println!("what base do you want? (must me smaller than 10)");
    io::stdin().read_line(&mut base_str).unwrap();
    // turning the string into numbers
    let base:i32 = base_str.trim().parse().unwrap();

    let basemax:i32 = base * base * base * base * base * base * base * base - 1;

    // taking input for your number
    println!("pick a number between 0 to {}",basemax);
    io::stdin().read_line(&mut num_str).unwrap();
    // turning the string into numbers
    let num:i32 = num_str.trim().parse().unwrap();
    // the main stuff

    // so if you have a chain of for loops...
    // thats how many digits you will get
    // and the var base acts like a base
    // unless it is bigger than 10 (v_v)

    for i in 0 ..base{
        for i2 in 0 ..base{
            for i3 in 0 .. base{
                for i4 in 0 .. base{
                    for i5 in 0 ..base{
                        for i6 in 0 ..base{
                            for i7 in 0..base{
                                for i8 in 0..base{
                                    count = count + 1;
                                    if count==num+1{
                                        // prints it out!
                                        println!("{}{}{}{}{}{}{}{}",i,i2,i3,i4,i5,i6,i7,i8);
                                }
                            }
                        }
                    }
                 }
                }
            }
        }

    }
}
