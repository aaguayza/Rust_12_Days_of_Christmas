//house a couple of verses in a const.
//the things number versus can be held in an array collection
//will need a loop to go through these
//the challenge is going to be how to make it repeat the last of the 
//may need to nest some loops like have a counter that will start at 1 and end at 12.
//the inner loop will have to loop a certian amount of times based on the counter which will then count up through the array.  


const ALL_DAYS: [&str; 7] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh"];
fn on_the_day(day:&str){
    print!("\nOn the {day} day of Christmas, my true love gave to me\n");
}

fn main() {
    for i in ALL_DAYS{
        on_the_day(i)
    }
}
