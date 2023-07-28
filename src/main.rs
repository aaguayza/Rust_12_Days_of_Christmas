//house a couple of verses in a const.
//the things number versus can be held in an array collection
//will need a loop to go through these
//the challenge is going to be how to make it repeat the last of the 
//may need to nest some loops like have a counter that will start at 1 and end at 12.
//the inner loop will have to loop a certian amount of times based on the counter which will then count up through the array.  


const ALL_DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelveth"];
const ALL_GIFTS: [&str; 12]= ["A partridge in a pear tree",
    "Two turtle doves and",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
    ];

fn main() {
    let mut counter = 0;
    for i in ALL_DAYS{
        print!("\nOn the {} day of Christmas\nMy true love gave to me\n", i);
        counter += 1;
        for number in (0..counter).rev(){
            print!("{}\n", ALL_GIFTS[number]);
        }
    }
}
