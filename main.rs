// BMI = weight in KG / height(squared) in meters
// UI in progress
use std::io;
use iced::widget::{button, column, text, Column};
fn kgconversion(weight: f32) -> f32 {
    let kg:f32 = 0.4535; // Pound per Kilogram
    weight * kg
}
fn meterconversion(height:f32) -> f32 {
    let meter:f32 = 0.0254; // Inches per Meter
    height * meter
}


fn main() {
    // Weight input
    println!("Please enter your weight in pounds: ");
    let mut weight_input: String = String::new();
    io::stdin().read_line(&mut weight_input).expect("wtf");
    let pounds: f32 = weight_input.trim().parse().expect("really?");

    // Height input
    println!("Now please enter your height in inches: ");
    let mut height_input: String = String::new();
    io::stdin().read_line(&mut height_input).expect("wtf");
    let inches: f32 = height_input.trim().parse().expect("really?");
    
    
    
    let weight_in_kg = kgconversion(pounds);
    let height_in_meters = meterconversion(inches);

    let bmi:f32 = weight_in_kg / (height_in_meters * height_in_meters);
    println!("Your BMI is {:.2}, wow!", bmi)
    
}

// UI below. Beware

/* Ice needs:
State — the state of your application
Messages — user interactions or meaningful events that you care about
View logic — a way to display your state as widgets that may produce messages on user interaction
Update logic — a way to react to messages and update your state
*/

// state
#[derive(Default)]
struct Counter {
    value: f32,
}
// User interactions on a button
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

// actual button
impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }
}

// button logic
impl Counter {
    // ...

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1.0;
            }
            Message::Decrement => {
                self.value -= 1.0;
            }
        }
    }
}
