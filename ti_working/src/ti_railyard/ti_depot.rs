use curl::easy::Easy;

mod ti_depot {
    // Basic functions that return basic things like site data
    mod basic {
        // Basic Page Request, returns raw HTML page data as a Vec:u8, accepting a url as a string.  
        fn preq(u: &str) -> Vec<u8> {
            // Create variables
            let mut s_h = Easy::new();
            let mut s_d = Vec::new(); 
            // Add the URL to the Easy handle. The program will fail here if the provided URL is not valid.
            s_h.url(u).unwrap();
            // Brackets here to ensure any borrows [Namely s_d] do not go out of scope or are used before their borrow is completed. 
            {
                // Transfer the Easy handler from one variable to another, this ensures that the Easy handle does not run out of time
                let mut s_t = s_h.transfer(); 
                // Set up the function that processes on another thread 
                s_t.write_function(|d| {
                    //Copy the incoming page data from the site into s_d 
                    s_d.extend_from_slice(d);
                    //Ensure that the incoming data has length. Not entirely sure why I need to do this, but the tutorial had it in so I left it in just to be safe. 
                    Ok(d.len())
                }).unwrap();
                // Start the page request and actually contact the server, will fail to networking-related problems.
                s_t.perform().unwrap(); 
            }
            //stdout().write(&s_d).unwrap();  | Debug function
            //Return the data as a Vec:8
            return s_d;
        }

    }

    //Functions that control depot, railyard will communicate to depot via this 
    mod ctrl {
        /* 
        Create an asyncronous thread that connects to a server and gathers site data
        Currently all done on the same thread because I don't know how threads work yet
        */
        
    }
    mod link {
        
    }
    mod scan {

    }
}