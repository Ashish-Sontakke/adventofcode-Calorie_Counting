
fn main() {
    let calories_list = concat!("
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    ");

   
    println!("the max calories are {}", get_max_calories(calories_list));


}


fn get_max_calories(calories_list: &str) -> usize {
    let mut this_user_calories = 0;
    let mut max_amount  = 0;
    let mut second_max = 0;
    let mut third_max = 0;

    // split string by new line
    for user_calories in calories_list.split("\n") {
        // check if split string is empty. 
        // this is not safe & better way to check than checking length
        if user_calories.len() > 4  {
            // split each line
            for calorie_count in user_calories.split('\n') {
                let calorie_value : usize = match calorie_count.trim().parse() {
                    Ok(value) => value,
                    Err(_) => 0,
                };
                this_user_calories += calorie_value;
            }
        }
        else {
            if this_user_calories > max_amount {
                third_max = second_max;
                second_max = max_amount;
                max_amount = this_user_calories;
            }
            else if this_user_calories > second_max {
                third_max = second_max;
                second_max = this_user_calories;
            }
            else if this_user_calories > third_max {
                third_max = this_user_calories;
            }
            this_user_calories = 0;
            println!("-------end-user-calories------- ")
        }
    }
    println!("the max amounts are  {}, {}, {}", max_amount,second_max,third_max);
    max_amount + second_max + third_max
}



