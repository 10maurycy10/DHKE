use clap::{Arg, Command};
use num_bigint::BigUint;
use num_traits::Num;
use num_traits::Zero;
use num_bigint::RandBigInt;

fn main() {
    let args = Command::new("dhke")
        .author("Maurycy, 10maurycy10@gmail.com")
        .about("A program to preform a DHKE (warning: this makes no attempt to zero memory)")
        .after_help("Diffie–Hellman key exchange (DHKE) is a way to establish \
        a shared secret between to party's without the need to secretly exchange\
        data. This secret can then be used as a key for a cipher. The algorithm \
        requires 2 non secret parameters, a large prime (p) and a small primitive root modulo p (g). \
        You can specify these on the command line with -p and -g.")
        .arg(Arg::new("hex").short('h').long("hex").help("Use hexadecimal."))
        .arg(Arg::new("modulus").short('p').long("mod").takes_value(true).value_name("NUMBER").help("A modulus to use for the exchange, this should be a large prime. (this value is *not* secret)"))
        .arg(Arg::new("gen").short('g').long("genorator").takes_value(true).value_name("NUMBER").help("A generator to use for the exchange, this should be a primitive root modulo p. (this value is *not* secret)"))
        .arg(Arg::new("a").short('s').long("seed").takes_value(true).value_name("NUMBER").help("A secret number for DHKE, ommit for random."))
        .get_matches();
    
    // find the base value.
    let hex = args.is_present("hex");
    let base = if hex {16} else {10};
    println!("using base {}", base);
    
    // defaults for DHKE paramiters
    let default_p = if !hex {"18446744073709551427".to_string()} else {"FFFFFFFFFFFFFF43".to_string()};
    let default_g = if !hex {"104".to_string()} else {"65".to_string()};
    
    let p = args.value_of("modulus").unwrap_or(&default_p);
    let g = args.value_of("gen").unwrap_or(&default_g);
    
    // Public constants
    let p = BigUint::from_str_radix(p, base).expect("invalid modulus");
    let g = BigUint::from_str_radix(g, base).expect("invalid generator");
    
//     // Sanity check for p
//     if ProbabPrimeResult::NotPrime == p.probab_prime(1024) {
//         println!("WARNING: the chosen p value is not a prime!!")
//     }
    
    println!("the parameters are p: {} g: {}",p.to_str_radix(base), g.to_str_radix(base));
    // secret value
    let mut rng = rand::thread_rng();
    let a = match args.value_of("a") {
        // parse the passed secret
        Some(s) => BigUint::from_str_radix(s, base).expect("invalid a value"),
        // generate a secret
        None => rng.gen_biguint_range(&BigUint::zero(),&p)
    };
    // Not so secret value to send to the other party.
    let ga = g.modpow(&a, &p);
    println!("g^a % p = {} (send this value to the other party.)", ga.to_str_radix(base));
    // get the vaule from the other party.
    let mut gb = BigUint::zero();
    loop {
        use std::io::{stdout,stdin};
        use std::io::Write;
        print!("enter value from other party: ");
        // flush buffer to ensure prompt is shown.
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim_end_matches('\n');
        println!("input: {:?}",buffer);
        match BigUint::from_str_radix(&buffer, base) {
            Err(_) => println!("invalid value entered"),
            Ok(num) => {
                gb = num;
                break;
            }
        }
    }
    let gba = gb.modpow(&a, &p);
    println!("The secret is {}",gba.to_str_radix(base));
}
