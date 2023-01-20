fn main() {
    let my_string = "this cat this bat this rat bat cat rat";
    let mut word:Vec<&str> = my_string.split(" ").collect();
    for i in 0..=word.len(){
        if i == word.len()-1{
            break;
        }else{
            for j in 0..=word.len(){
                if j >= word.len()-1 {
                        break;
                    }else if j+1 < word.len() && i < word.len() && i != j+1{
                        if  word[i] == word[j+1] {
                            word.remove(j+1);
                        }
                    }
            }
        }
    }
    println!("unique_word : {:?}",word);
    println!("count : {:?}",word.len());
} 
#[cfg(test)]
mod tests {
    #[test]
    fn unique_word() {
        let my_string = "this cat this bat this rat bat cat rat";
    let mut word:Vec<&str> = my_string.split(" ").collect();
    for i in 0..=word.len(){
        if i == word.len()-1{
            break;
        }else{
            for j in 0..=word.len(){
                if j >= word.len()-1 {
                        break;
                    }else if j+1 < word.len() && i < word.len() && i != j+1{
                        if  word[i] == word[j+1] {
                            word.remove(j+1);
                        }
                    }
            }
        }
    }
    let result = word.len();
    assert_eq!(result, 4);
    }
}