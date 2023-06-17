fn main() {
    let image_width = 256;
    let image_height = 256;
    
    let mut file = String::new();

    file.push_str(&format!("P3\n{image_width} {image_height}\n255\n"));

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.25;

            let ir: u8 = (255.999 * r) as u8;  
            let ig: u8 = (255.999 * g) as u8;  
            let ib: u8 = (255.999 * b) as u8;  
            
            file.push_str(&format!("{ir} {ig} {ib}\n"))
        }
    }



    println!("{}", file);
}
