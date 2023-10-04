#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Below is the function stub for deal. Add as many helper functions
    as you like, but the deal function should not be modified. Just
    fill it in.
    
    Test your code by running 'cargo test' from the war_rs directory.
*/
fn deal(shuf: &[u8; 52]) -> [u8; 52]
{
    let (mut p1, mut p2, mut w_chest) = (Vec::new(), Vec::new(), Vec::new()); //intializing three vectors, player 1, player 2 and war chest temp storage
    for i in (0..52).rev() { //for loop to assign 26 cards to each player
        if i%2==0 {
            p1.push(shuf[i]);
        } else {
            p2.push(shuf[i]);
        }
    }        
    for i in 0..p1.len() { //loop to change values of all the 1s to 14s in p1
        if p1[i] == 1 {
            p1[i] = 14;
        }
    }
    for i in 0..p2.len() { //loop to change values of all the 1s to 14s in p1
        if p2[i] == 1 {
            p2[i] = 14;
        }
    }
    let mut arr1 = [0; 52]; //creating an array to output player 1's cards if it wins
    for i in 0..p1.len() {
        arr1[i] = p1[i];
    }
    let mut arr2 = [0; 52]; //creating an array to output player 2's cards if it wins
    for i in 0..p2.len() {
        arr2[i] = p2[i];
    }
    let mut arr3 = [0; 52]; //creating an array to output war chest
    for i in 0..w_chest.len() {
        arr3[i] = w_chest[i];
    }
            while p1.len() > 0 && p2.len() > 0 // while both players have cards
            {
            if p1[0]>p2[0] // player 1 wins a round
            {
                w_chest.push(p1[0]);
                w_chest.push(p2[0]);
                w_chest.sort_by(|x, y| y.cmp(x)); // sorting in descending order
                p1.extend(w_chest.clone());
                w_chest.clear(); // clearing war chest for reuse
            }
            else if p1[0]<p2[0] // player 2 wins a round
            {
                w_chest.push(p1[0]);
                w_chest.push(p2[0]);
                w_chest.sort_by(|x, y| y.cmp(x)); // sorting in descending order
                p2.extend(w_chest.clone());
                w_chest.clear(); // clearing war chest for reuse
            }
            else
            {
                if p1.len()>=2 && p2.len()>=2 // WAR
                {
                    w_chest.push(p1[0]);
                    w_chest.push(p2[0]);
                    w_chest.push(p1[1]);
                    w_chest.push(p2[1]);
                    w_chest.sort_by(|x, y| y.cmp(x)); //sorting in descending order
                    
                    p1.remove(1); // removing card from p1 at second index
                    p2.remove(1); // removing card from p2 at second index

                }
                else if p1.len() == 1 && p2.len() > 0 { // edge case if player 1 has only 1 card left
                    p2.push(p2[0]);
                    p2.push(p1[0]);
                }
                else { // if player 2 has only 1 card
                    p1.push(p2[0]);
                    p1.push(p1[0]);
                }
            }
            p1.remove(0); // removing card from p1 at first index
            p2.remove(0); // removing card from p2 at first index
        }

        if p2.len()==0 && p1.len() > 0 { // if player 1 wins
            w_chest.sort_by(|x, y| y.cmp(x)); // sorting by descending order
            p1.extend(w_chest.clone());  // putting the warchest in player 1
                
            for i in 0..p1.len() { //converting all the 14s back into 1s
                if p1[i] == 14 {
                    p1[i] = 1;
                }
            }
            for i in 0..p1.len() { // loop to push all the elements of the vector of p1 into an array
                arr1[i] = p1[i];
            } 
            arr1 // output array
            
        }
    
        else if p1.len()==0 && p2.len() > 0 // if player 2 wins
        {
            w_chest.sort_by(|x, y| y.cmp(x)); // sorting by descending order
            p2.extend(w_chest.clone());  // putting the warchest in player 2
                
            for i in 0..p2.len() { //converting all the 14s back into 1s
                if p2[i] == 14 {
                    p2[i] = 1;
                }
            } 
            for i in 0..p2.len() { // loop to push all the elements of the vector of p2 into an array
                arr2[i] = p2[i];
            } 
            arr2 // returning array
        }
        else
        {
            w_chest.sort_by(|x, y| y.cmp(x)); // where both p1 and p2 are empty due to constant wars
            arr3[..w_chest.len()].copy_from_slice(&w_chest[..]);  //pushing into an array from warchest vector
            for i in 0..w_chest.len() {
                if w_chest[i] == 14 { // changing 14s to 1s
                    arr3[i] = 1;
                }
            } 
            arr3 // returning array
        }
    }
#[cfg(test)]
#[path = "tests.rs"]
mod tests;

