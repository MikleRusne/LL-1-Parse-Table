#![allow(dead_code)]
pub mod cfgtools{
    use prettytable::{Table, Row, Cell};
    use std::collections::HashMap;


fn get_first<'a>(
    target: &'a str,
    productions: &'a [Vec<&'a str>],
    alphabet: &'a Vec<&'a str>,
    variables: &'a Vec<&'a str>,
)-> Vec<&str>{
    let mut first_characters = Vec::new();
    
    //For debug purposes
    let mut target_productions= Vec::new();
    
    //Check productions
    for i in 0..productions.len(){
        if productions[i].first().unwrap() == &target{
            // println!("Found {} in {:?}",&target, &productions[i]);
            // println!("{:?}", &productions[i]);
            target_productions.push(i.clone());
            
            //We found it on the right side
            //Check the first character
            //Ignore epsilon
            if &productions[i][1] == &"epsilon" {
                continue
            }

            if alphabet.contains(&productions[i][1]){
                first_characters.push(productions[i][1].clone());
            }
            else if variables.contains(&productions[i][1]){
                // println!("{} is a variable", &productions[i][1]);
                first_characters.append(&mut get_first(&productions[i][1], &productions, &alphabet, &variables))
            }
        }
    }
    // println!("Found target {} in production {:?}", &target, &target_productions);
    // println!("First characters {:?}", &first_characters);
    first_characters.sort_unstable();
    first_characters.dedup();
    first_characters
}

fn is_nullable(
    target: &str,
    productions: &[Vec<&str>],
)->bool{
    for i in 0..productions.len(){
        if productions[i].first().unwrap() == &target{
            if productions[i][1..].contains(&"epsilon"){
                // println!("{} is nullable", &target);
                return true;
            }
        }
    }
    // println!("{} is not nullable", &target);
    return false;
}


fn get_follow<'a>(
    target: &'a str,
    productions: &'a [Vec<&'a str>],
    alphabet: &'a Vec<&'a str>,
    variables: &'a Vec<&'a str>,
    start_variable: &'a str
)-> Vec<&str>{
    let mut follow_characters = Vec::new();
    
    
    //For debug purposes
    let mut target_productions= Vec::new();
    
    //If it is start variable, include $
    if &start_variable==&target {
        follow_characters.push("$");
    }
    //Check productions
    for i in 0..productions.len(){
        if productions[i][1..].contains(&target) {
            // println!("Found {} on right side in {:?}",&target, &productions[i]);
            // println!("{:?}", &productions[i]);
            let index = productions[i][1..].iter().position(|&r| &r==&target).unwrap();
            // println!("At position {:?}",index);
            //Check the very next thing
            
            //If it is at the last position, include the follow of left side
            if index+2>=productions[i].len(){
                // println!("It is at the last position");
                //Check if the first character is not the same as target
                if &productions[i][0]!= &target{
                    follow_characters.append(&mut get_follow(
                        &productions[i][0],
                        &productions,
                        &alphabet,
                        &variables,
                        &start_variable
                    ));
                }
                continue;
            }
            //If it is an alphabet, include it in the follow set
            if index+2 < productions[i].len() && alphabet.contains(&productions[i][index+2]){
                // println!("The next character is an alphabet, {}", &productions[i][index+2]);
                follow_characters.push(&productions[i][index+2]);
            }
            if variables.contains(&productions[i][index+2]){
                // println!("The next character is a variable, {}", &productions[i][index+2]);
                follow_characters.append(&mut get_first(&productions[i][index+2], &productions, &alphabet, &variables));
                //If the variable can be nulled, modify the rule to be
                let mut null_check_iterator = index+2;
                // println!("starting null checks at {}", &productions[i][null_check_iterator]);
                //If the next character is nullable, add the next next character into it
                if is_nullable(&productions[i][null_check_iterator], &productions){
                    while 1==1{
                        // println!("{} can be nulled, adding its first.", &productions[i][null_check_iterator]);
                        //Add the first of the very next variable
                        if null_check_iterator+1>= productions[i].len() {
                            if &productions[i][0]!= &target{
                                follow_characters.append(&mut get_follow(
                            &productions[i][0],
                                    &productions,
                                    &alphabet,
                                    &variables,
                                    &start_variable
                                ));
                            }
                            break;
                        }
                        follow_characters.append(&mut get_first(&productions[i][null_check_iterator+1], 
                            &productions, &alphabet, &variables));
                        //If the next one can also be nullable, increase iterator
                        null_check_iterator+=1;
                        if !(null_check_iterator< productions[i].len()) || is_nullable(&productions[i][null_check_iterator], &productions){
                            
                        }else{
                            null_check_iterator+=&productions[i].len();
                        }
                    
                    }

                }
            }
            target_productions.push(i.clone())
        }
    }
    // println!("Found target {} in production {:?}", &target, &target_productions);
    // println!("First characters {:?}", &first_characters);
    follow_characters.sort_unstable();
    follow_characters.dedup();
    follow_characters

}

//Helper
fn vusize_to_string(
    vec: &Vec<usize>
)-> String{

    let x = vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    // println!("Converted be like: {}",&x);
    x
}
    pub fn create_parsing_table(
        terminals: &Vec<&str>,
        variables: &Vec<&str>,
        productions: &[Vec<&str>],
        start_variable: &str
    )-> (){
        //Create a two dimensional Array
        let mut table: HashMap<&&str, HashMap<&&str, Vec<usize>>> = variables.iter().map(
            |x| (x,
                terminals.iter().map( // For each terminal, create a vector
                    |y| ( if y!=&"epsilon" {y} else {&"$"}, vec![]) //Reusing epsilon for $
                )
                .collect()
            )
        ).collect();
    
        //Enumerate each production
        productions.iter().enumerate().for_each(|(i, production)| {
            //Get the first of the right hand side
            //  Simplify
            //      If first character is a terminal, then it is the first
            if terminals.contains(&production[1]) {
                //But it could be epsilon, which is a special case
                if &production[1]==&"epsilon"{
                    // println!("First of production:{}, is epsilon, will add follow", &i);
                    let follows = get_follow(&production[0], &productions, 
                        &terminals, &variables, &start_variable);
                    //In the hash table, in the row of left side,
                    table.entry(&production[0]).and_modify(
                        //Against each terminal in follow, insert into vector
                        |v| {
    
                            //Against each entry in hashmap, check if it is in follows and does not exist already
                            for x in follows{
                                let y = v.get_mut(&x).unwrap();
                                if !y.contains(&i){
                                    y.push(i)
                                }
                                
                           }
                        }
                    );
                }
                else{
                    // println!("Setting first of production:{}, as {}", &i, &production[1]);
                    //In the hash table, in the row of left side,
                    table.entry(&production[0]).and_modify(
                        |v: &mut HashMap<&&str, Vec<usize>>|{
                            // println!("Inserting in v: {:?}", &v);
                            // in the column of that terminal,
                            v.entry(&production[1]).and_modify(
                                //insert production number i into the vector
                                |w: &mut Vec<usize>| {
                                    if !w.contains(&i){
                                        w.push(i);
                                    }
                                }    
                            );
                        }
                    );
                }
            }
            else
            {
                //It's not a terminal
                //  Get firsts
                //  Add production number to each first
                let firsts = get_first(&production[1], &productions, 
                    &terminals, &variables);
                //In the hash table, in the row of left side,
                table.entry(&production[0]).and_modify(
                    //Against each terminal in firsts, insert into vector
                    |v| {
                            for x in firsts{
                            let y = v.get_mut(&x).unwrap();
                            if !y.contains(&i){
                                y.push(i)
                            }
                       }
                    }
                );
            }
        });
    
    
        //Convert to pretty table
        let mut pretty_table = Table::new();
        let mut columns_vector = vec![Cell::new("")];
        for &term in terminals{
            columns_vector.push(
                Cell::new(if &term!=&"epsilon" {&term} else {"$"})
            );
        }
        
        pretty_table.add_row(Row::new(
            columns_vector
        ));
        for ele in &table{
            // println!("ele: {:?}", &ele);
            //Add the row title
            let mut cells = vec![Cell::new(ele.0)];
            //Add the numbers
            for &term in terminals.iter(){
                let key = if &term== &"epsilon" {"$"} else {&term}; 
                // println!("Trying key {} in {:?}", &key, &ele);
                cells.push(Cell::new(&vusize_to_string(&ele.1[&key])));
            }
            
            pretty_table.add_row(
                Row::new(cells)
            );
        }
        pretty_table.printstd();
    
    }
    
    
}
