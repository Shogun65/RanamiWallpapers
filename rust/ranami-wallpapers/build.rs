

fn main(){

    let mut res = winres::WindowsResource::new();
    
    res.set_icon("../assets/RanamiIcon.ico");

    res.compile().unwrap();

}