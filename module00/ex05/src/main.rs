fn  is_leap_year(year: u32) -> bool
{
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    {
        return true;
    }
    false
}

fn  num_days_in_month(year: u32, month: u32) -> u32
{
    if is_leap_year(year) && month == 2
    {
        29
    }
    else if !is_leap_year(year) && month == 2
    {
        28
    }
    else if (month <= 7 && month % 2 != 0) || (month > 7 && month % 2 == 0)
    {
        31
    }
    else
    {
        30
    }
}

fn  get_days_vector() -> Vec<String>
{
    let mut vector: Vec<String> = Vec::new();

    vector.push("Monday".to_string());
    vector.push("Tuesday".to_string());
    vector.push("Wednesday".to_string());
    vector.push("Thursday".to_string());
    vector.push("Friday".to_string());
    vector.push("Saturday".to_string());
    vector.push("Sunday".to_string());
    vector
}

fn  get_months_vector() -> Vec<String>
{
    let mut vector: Vec<String> = Vec::new();

    vector.push("January".to_string());
    vector.push("February".to_string());
    vector.push("March".to_string());
    vector.push("April".to_string());
    vector.push("May".to_string());
    vector.push("June".to_string());
    vector.push("July".to_string());
    vector.push("August".to_string());
    vector.push("September".to_string());
    vector.push("October".to_string());
    vector.push("November".to_string());
    vector.push("December".to_string());
    vector
}

fn main() 
{
    let mut days_per_month: u32;
    let mut days: u32;
    let mut months: u32;
    let mut years: u32;
    let days_of_week: Vec<String>;
    let months_of_year: Vec<String>;

    days_of_week = get_days_vector();
    months_of_year = get_months_vector();
    days = 1;
    months = 1;
    years = 1;
    days_per_month = num_days_in_month(years, months);
    while years <= 2024
    {
        for day_temp in &days_of_week
        {
            if years == 2025
            {
                break
            }
            if days == 13 && day_temp == "Friday"
            {
                println!("Friday, {} 13, {}", months_of_year[months as usize - 1], years);
            }
            if days < days_per_month
            {
                days += 1;
            }
            else if days == days_per_month && months == 12
            {
                months = 1;
                days = 1;
                years += 1;
                days_per_month = num_days_in_month(years, months);
            }
            else
            {
                months += 1;
                days = 1;
                days_per_month = num_days_in_month(years, months);
            }
        }
    }
}
