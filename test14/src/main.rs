fn main() {
    let test_score = 85; 
    
    let result = get_status(test_score);
    
    println!("For a score of {}, the result is: {}", test_score, result);
    println!("For a score of 45, the result is: {}", get_status(45));
}


fn get_status(score: i32) -> String {
    let label = if score >= 90 {
        "Excellent"
    } else if score >= 75 {
        "Good"
    } else if score >= 50 {
        "Pass"
    } else {
        "Fail"
    };

    match label.chars().next() {
        Some('E') => "Top".to_string(),
        Some('G') => "Decent".to_string(),
        Some('P') => "Basic".to_string(),
        _ => "None".to_string()
    }
}