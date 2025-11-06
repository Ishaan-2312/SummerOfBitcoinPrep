fn main() {
    let arr = vec![3,4];
    let target = 5;
    let (subset, change) = find_min_sum_exceeding_target(&arr, target);
    println!("Included elements: {:?}, Change: {}", subset, change); 

}

fn find_min_sum_exceeding_target(arr : &[usize],target : usize) -> (Vec<usize> , usize){
    let n = arr.len();
    let max_sum : usize = arr.iter().sum();
    let dp_size = max_sum + 1;

    let mut dp : Vec<Option<Vec<usize>>> = vec![None;dp_size];
    dp[0] = Some(vec![]);

    for (i,&val) in arr.iter().enumerate(){
        for  sum in (0..=max_sum - val).rev(){

            if dp[sum].is_some() && dp[sum + val].is_none(){
                let mut new_subset = dp[sum].clone().unwrap();
                new_subset.push(i);
                dp[sum + val ]= Some(new_subset);
            }
        }
    }
    for sum in target..=max_sum {
        if let Some(included_indices) = &dp[sum] {
            let included_values = included_indices.iter().map(|&idx| arr[idx]).collect::<Vec<_>>();
            let change = sum - target;  // sum - target since sum is >= target
            return (included_values, change);
        }
    }

    // no subset found that meets or exceeds target
    (vec![], target)


}



