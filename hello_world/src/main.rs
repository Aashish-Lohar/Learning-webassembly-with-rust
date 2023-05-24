fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    // variable declarations, by default rust variables are immutable to make them mutable we need to mention "mut" before variable name
    let mut total = add(40, 20);
    let mut free_shipping = false;

    if total > 50 {
        println!("You are eligible for free shipping");
        free_shipping = true;
    } else if total > 20 {
        println!("If you add more items, You will eligible for free shipping");
    } else {
        println!("No free shipping");
    }

    // match expression
    // match free_shipping {
    //     true=>total=total+0,
    //     false=>total=total+50
    // }
    total = match free_shipping {
        true => total + 0,
        false => total + 50,
    };

    match total {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match found"),
    }

    println!("Total:{:?}", total);

    let item: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", item);

    // vector decalarations
    let vector_items = vec![1, 2, 3, 4, 5];
    let mut vector_items2 = Vec::new();
    vector_items2.push(1);
    vector_items2.push(2);
    vector_items2.push(3);
    vector_items2.push(4);
    vector_items2.push(5);

    println!("{:?}", vector_items);
    println!("{:?}", vector_items2);
}
