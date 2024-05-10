enum Light {
    Bright,
    Dull
}

fn display_light(light: &Light){
    match light {
        Light::Bright => println!("bright light"),
        Light::Dull => println!("dull light")
    }
}

fn main(){
    let my_light = Light::Dull;
    display_light(&my_light);
    display_light(&my_light);
}