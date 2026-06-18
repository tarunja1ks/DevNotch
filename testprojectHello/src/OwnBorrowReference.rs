pub fn main() {
    let model = String::from("Intel CoreAadit");
    let active_model = model;
    
    let battery = 87.5_f64;
    let cpu = 45.2_f64;
    
    print_stats(&active_model, &battery, &cpu);
    
    let mut temperature = 72.0_f64;
    apply_throttle(&mut temperature);
    println!("{}", temperature);


    let boosted = boost_clock(cpu);
    println!("{}", boosted);

    let mut active_model=active_model;
    let label = create_label(&mut active_model);
    println!("{}", label);

    shutdown(active_model);
}

fn print_stats(model: &String, battery: &f64, cpu: &f64) {
    println!("Model {}, Battery Percentage {}, CPU Temp {}", model, battery, cpu)
    
}

fn apply_throttle(temp: &mut f64) {
    *temp+=5.0;
}

fn shutdown(model: String) {
    println!("Model is shut down: {}", model);
}

fn boost_clock(cpu: f64) -> f64 {
    cpu+10.0
}

fn create_label(model: &mut String) -> String {
    format!("This is the model: {}", model)
}