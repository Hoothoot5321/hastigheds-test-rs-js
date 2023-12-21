use std::time::Instant;

fn main() {
    for i in 5..12 {

        let start = Instant::now();

        let amount:i64=5*10_i64.pow(i);
        let sum:i64 = 0;
        for a in 0..amount {
            let temp = a/2;
            sum+=temp;
        }

        let elapsed = (start.elapsed().as_millis() as f64)/1000.0;

        println!("Pow: {}\nSum: {}\nTime: {}\n",i,sum,elapsed);
    }
}
