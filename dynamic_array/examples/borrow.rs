fn main(){
    let mut vrr: Vec<&str> = vec!["Hello", "World", "Coders"];//vrr is
    write_vrr(&mut vrr);
    println!("vrr={:?}", vrr);
}

fn write_vrr(vrr2: &mut Vec<&str>){
    vrr2.push("Fellow"); //error: cannot borrow `*vrr2` as mutable, as it is behind a `&` reference
    println!("vrr2={:?}", vrr2);
}