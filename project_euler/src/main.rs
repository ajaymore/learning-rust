// fn question_one() -> i32 {
//     let mut sum = 0;
//     for number in 1..1000 {
//         if number % 3 == 0 || number % 5 == 0 {
//             sum = sum + number;
//         }
//     }
//     sum
// }

// fn question_two() -> i32 {
//     let mut first = 1;
//     let mut second = 2;
//     let mut sum = 2;
//     while second < 4_000_000 {
//         let temp = first;
//         first = second;
//         second = second + temp;
//         if second < 4_000_000 && second % 2 == 0 {
//             sum = sum + second;
//         }
//     }
//     sum
// }

fn question_three(num: i64) -> i64 {
    let mut given_num = num;
    let mut iter = 0;
    while given_num > 1 {
        iter = iter + 1;
        let mut i = 2;
        while given_num % i != 0 {
            iter = iter + 1;
            i = i + 1;
            if i == given_num {
                break;
            }
        }
        if i == given_num {
            break;
        } else {
            given_num = given_num / i;
        }
    }
    println!("{}", iter);
    given_num
}

fn main() {
    // let answer = question_one();
    // println!("The answer is {}", answer);

    // let answer = question_two();
    // println!("The answer is {}", answer);

    let answer = question_three(600851475143);
    println!("The answer is {}", answer);
}
