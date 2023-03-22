// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.


// let optional_value = Some(String::from("rustlings"));
// // Make this an if let statement whose value is "Some" type
// if let Some(value) = optional_value {
//     println!("the value of optional value is: {}", value);
// } else {
//     println!("The optional value doesn't contain anything!");
// }

// let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
// for x in 1..10 {
//     optional_values_vec.push(Some(x));
// }

// // make this a while let statement - remember that vector.pop also adds another layer of Option<T>
// // You can stack `Option<T>`'s into while let and if let
// while let Some(Some(value)) = optional_values_vec.pop() {
//     println!("current value: {}", value);
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            println!("the value of optional_target {}", word);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let

        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(Some(integer), Some(range));
            range -= 1;
        }
    }

    //     while let integer = optional_integers.pop() {
    //         assert_eq!(integer, range);
    //         range -= 1;
    //     }
    // }
}
