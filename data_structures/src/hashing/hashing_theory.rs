#[allow(unused)]
pub fn search_query_loops(target: &[u8], query: &[u8]) {
    let mut vec = Vec::with_capacity(target.len());
    for i in query {
        let mut counter = 0;
        for j in target {
            if i == j {
                counter += 1;
            }
        }
        vec.push(counter);
    }
    println!("{:?}", vec);
} // NAIVE WAY OF USING FOR LOOPS

/*
LEARNING: 10^8 operations takes 1 second to execute. if target and query are each 10^5 size, then overall size will be 10^10, which will take 100 seconds to execute. NOT A GOOD CODE.
*/

// HASHING METHOD
