# word_finder

English is not my first language. Therefore while reading an ancient philosophy book I encountered many English words I do not know the meaning. So reading plan is I will write the down the meaning of the word in my native language on the pages of that book. Surprisingly I found many words that I do not the meaning. So I started writing down the meaning of all these words on the page they appear. 

I would not write down their meanings if the book is one time reading, but I plan to read this book over and over again. As I am reading the book further I am finding that many words repeat so I wanted to avoid writing down the meaning these repeated words. Rather I want to write the only the page number where I first encountered the these repeated words. This is why I made this command line tools using Rust and postgresql. 

This tool basically does three things: inserting the words to database with page number, searching the words, and counting the total words in the database. 

To insert the words simply run the following command from this tool directory: `cargo run i`, then enter the word along with page number. This insertion process continues unless we enter the char `q` as the word. Entering the `q` terminates the insertion process. If a word is already in the database we can't that word again, it will panick.

To search the word already added to the database simply run the following command: `cargo run s`, enter the word to be searched, this whole process continues untill we enter the search word as `q`. `q` terminates the searching process. Searching always returns the word along with the page number if found.

To see how many words are in the database simply run the following command from the terminal `cargo run t`. 
