/* fn main(){
    let arr: [&str; 3] = ["Hello", "World", "Coders"];
    write_arr(arr);//array directly pass
    println!("arr={:?}",arr);
}

fn write_arr(mut arr1:[&str; 3]){
    arr1[0]="Fellow";
    println!("arr1={:?}",arr1);
}
*/

fn main(){
    let mut arr: [&str; 3] = ["Hello", "World", "Coders"];
    write_arr(&mut arr);
    println!("arr={:?}",arr);
}

fn write_arr(arr1:&mut [&str; 3]){
    arr1[0]="Fellow";
    println!("arr1={:?}",arr1);
}
