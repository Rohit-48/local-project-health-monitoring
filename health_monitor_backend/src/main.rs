use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};



fn main() {
    println!("Hello Health Monitor Backend!");
}
 

// User Data 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello Health Monitor Backend!");
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct UserData{
    id:u64,
    name: String,
    age: u32,
    height: f32,
    weight: f32,
    gender: Gender,
    activity_level: ActivityLevel,
    goal: String,
    health_data: HealthData,
    email: String,
    password: String,
}

// Gender
enum Gender{
    Male,
    Female,
    Other,
}
// Activity Level
enum ActivityLevel{
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    SuperActive,
}

// health graph data
struct HealthGraph{
    date: String,
    calories: f32,
    protein: f32,
    carbs: f32,
    fat: f32,
}

// water intake
struct WaterIntake{
    date: String,
    amount: f32,
}

struct foodInput{
    food_name: String,
    unit: u64,
}
// Calculate BMR
fn calculate_bmr(user_data: &UserData) -> f32{
    let bmr = 0.0;
    match user_data.gender{
        Gender::Male => bmr = 88.362 + (13.397 * user_data.weight) + (4.799 * user_data.height) - (5.677 * user_data.age),
        Gender::Female => bmr = 447.593 + (9.247 * user_data.weight) + (3.098 * user_data.height) - (4.330 * user_data.age),
        Gender::Other => bmr = 0.0,
    }
    bmr
}

// Calculate activity level
fn calculate_activity(user_data: &UserData) -> f32{
    let activity_level = 0.0;
    match user_data.activity_level{
        ActivityLevel::Sedentary => activity_level = 1.2, // Little or no exercise, desk job
        ActivityLevel::LightlyActive => activity_level = 1.375, // Light exercise, 1-3 days/week
        ActivityLevel::ModeratelyActive => activity_level = 1.55, // Moderate exercise, 3-5 days/week
        ActivityLevel::VeryActive => activity_level = 1.725, // Hard exercise, 6-7 days/week
        ActivityLevel::SuperActive => activity_level = 1.9, // Very hard exercise, physical job or 2x training
    }
    activity_level
}

// Calculate TDEE
fn calculate_tdee(user_data: &UserData) -> f32{
    let tdee = 0.0;
    tdee = clculate_bmr(user_adata) * calculate_activity(user_data);
    tdee
}

// Calculate Macros
fn calculate_macros(user_data: &UserData) -> (f32, f32, f32){
    let protien = 0.0;
    let carbs = 0.0;
    let fat = 0.0;
    match user_data.goal{
        "lose_weight" => {
            protien = 0.3 * tdee;
            carbs = 0.4 * tdee;
            fat = 0.3 * tdee;
        }
        "gain_weight" => {
            protien = 0.35 * tdee;
            carbs = 0.45 * tdee;
            fat = 0.2 * tdee;
        }
        "deficit" => {
            protien = 0.3 * tdee;
            carbs = 0.4 * tdee;
            fat = 0.3 * tdee;
        }
    }
    (protien, carbs, fat)
}

// Calculate Calories
fn calculate_calories(user_data: &UserData) -> f32{
    let calories = 0.0;
    calories = calculate_tdee(user_data) + calculate_macros(user_data);
    calories
}

// Calculate Protein
fn calculate_protein(user_data: &UserData) -> f32{
    let protein = 0.0;
    protein = calculate_macros(user_data).0;
}

// Calculate Carbs
fn calculate_carbs(user_data: &UserData) -> f32{
    let carbs = 0.0;
    carbs = calculate_macros(user_data).1;
    carbs
}

// Calculate Fat
fn calculate_fat(user_data: &UserData) -> f32{
    let fat = 0.0;
    fat = calculate_macros(user_data).2;
    fat
}




