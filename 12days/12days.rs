fn main() {
  let gifts = [
    "Partridge in a pear tree", 
    "Turtle doves", 
    "French hens", 
    "Collie birds", 
    "Golden rings", 
    "Geese a-laying", 
    "Swans a-swimming", 
    "Maids a-milking", 
    "Pipers piping", 
    "Drummers drumming", 
    "Lords a-leaping", 
    "Ladies dancing",
  ];
  for i in 1..=12 {
    let day = if i == 1 {"st"} else if i == 2 {"nd"} else if i == 3 {"rd"} else {"th"};
    println!("\n On the {0}{1} day of christmas, my true love gave to me", i, day);
    for e in (1..=i).rev() {
      let and = if e == 1 && i > 1 {"and"} else {""};
      println!(" {} {} {}", and, e, gifts[e-1]);
    };
  };
}