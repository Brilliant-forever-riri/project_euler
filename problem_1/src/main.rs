// fn main() {
//     let mut vec = vec![]; //don't put a vector inside a loop
//     for i in 1..1000{
//         if i%3 == 0 || i%5 == 0 {
//             vec.push(i);  
//         }
//     }
//     let mut sum = 0;
//     for i in vec.iter(){
//         sum = sum+i;
//     }
//     println!("{}", sum);
// }

// best version of the code is below

fn main() {
    let mut sum = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0{
            sum+= i;
        }
    }
    println!("{sum}");
    
    
}


