fn main() {
    let a = 8;      // coefficient a
    let b = 2;      // coefficient b
    let p = 17;      // prime modulus

    // Iterate over all possible x-coordinates
    for x in 0..p {
        let x_cubed = (x * x % p * x % p) % p;
        let rhs = (x_cubed + a * x % p + b) % p;

        // Find y^2 mod p using brute force
        for y in 0..p {
            if (y * y % p) == rhs {
                println!("Point ({}, {})", x, y);
                // Also consider the point (x, p - y) if y != 0
                if y != 0 {
                    println!("Point ({}, {})", x, p - y);
                }
                break; // Only need one point per x
            }
        }
    }
}
