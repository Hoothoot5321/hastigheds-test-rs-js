
for (let i = 5; i < 12; i++) {

    console.time("concatenation");
    let amount = 5 * Math.pow(10, i);

    let sum = 0
    for (let a = 0; a < amount; a++) {
        let temp = i / 2
        sum += temp
    }
    console.log("Pow: ", i)
    console.log("Sum: ", sum)
    console.timeEnd("concatenation")
}


