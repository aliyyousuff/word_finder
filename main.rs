use std::io::stdin;
use std::env;
use postgres::{Client, NoTls, Error};


fn main() ->Result<(), Error>
{
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS dictionary (
            id              SERIAL PRIMARY KEY,
            name            TEXT NOT NULL UNIQUE,
            page            INT
            )
    ")?;


    let args: Vec<String> = env::args().collect();

    let ins_se = &args[1];

    if ins_se.to_ascii_lowercase().eq(&"i")
    {
        let mut insertability = true;
        

        while insertability
        {
            println!("Enter the word or q to quit the entering word: ");
            let mut word = String::new();

            stdin().read_line(&mut word)
    	        .ok()
                .expect("Failed to read line");
            
            word.pop();

            
            if word.to_ascii_lowercase().ne(&"q")
            {
                //insertability = true;
        
                println!("Enter the page number: ");
                let mut page = String::new();
        
                //taking input from the user.
                stdin().read_line(&mut page)
                    .ok()
                    .expect("Failed to read line");
                
                // converting the entered string to integer
                let page_no:i32 = page.trim().parse().expect("Input not an integer");

                // Insert the data into the database here:
                client.execute(
                    "INSERT INTO dictionary (name, page) VALUES ($1, $2)",
                    &[&word, &page_no],
                )?;
            }
            else
            {
                insertability = false;
            }
            
        }
    }
    else if ins_se.to_ascii_lowercase().eq(&"s")
    {
        let mut search_continuation = true;

        while search_continuation
        {
            let mut searching_word = String::new();

            println!("Enter the searching word or q to quit searching:");

            //taking input from the user.
            stdin().read_line(&mut searching_word)
                .ok()
                .expect("Failed to read line");

            searching_word.pop();
            let mut found = false;

            if searching_word.to_ascii_lowercase().ne(&"q")
            {        
                // Search here from the database
                for row in client.query("SELECT id, name, page FROM dictionary", &[])? 
                {
                    let mut word: String = row.get(1);
                    let page_number: i32 = row.get(2);

                    //remove the newline character from the string.
                    word.pop();

                    if word.eq(&searching_word)
                    {
                        println!("Found the word: {}, page number: {}",word, page_number);
                        found = true;
                    }
                }
                if found == false
                {
                    println!("{} is not found", searching_word);
                }
             
            }
            else 
            {
                search_continuation = false; 
            }
        }
    }

    else if ins_se.to_ascii_lowercase().eq(&"t")
    {
        let mut total_word = 0;
        for _row in client.query("SELECT id FROM dictionary", &[])?
        {
            total_word = 1 + total_word;
        }

        println!("Total word the database contains {}.",total_word);

    }

    else
    {
        panic!("Please enter the correct word for either to insert(i) or search(s)")
    }

    Ok(())
}
    



    
