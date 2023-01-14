extern crate vi;

use vi::vni;

fn main() {
    let inputs = vec![vec!['v', 'i', 'e', 't', '5', '6'], vec!['n', 'a', 'm']];

    let mut result = String::new();
    for input in inputs {
        vni::transform_buffer(input.iter().cloned(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints "việt nam "
}
