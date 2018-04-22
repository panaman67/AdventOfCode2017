use std::io::{BufRead, BufReader};
use std::fs::File;

fn main()
{
    let input_file = File::open("input_day6.txt").expect("Error opening file");
    let mut buf = BufReader::new(input_file);

    let mut s = String::new();
    buf.read_line(&mut s).unwrap();

	let line : Vec<&str> = s.trim().split('\t').collect();

	let mut row : Vec<i8> = Vec::new();
	for x in 0..line.len()	   
    {
	    row.push(String::from(line[x]).parse::<i8>().unwrap());
	}

    let mut seen : Vec<Vec<i8>> = Vec::new();
    seen.push(row.clone());

    let mut cnt = 1;
    let mut max : (usize, i8);
    println!("{:?}", row);
    loop
    {
        // reset max
        max = (0, std::i8::MIN);
        // loop through 'row' vec to find largest value and position
        for (index, num) in row.iter().enumerate()
        {
            // if number at index is bigger than current max
            if *num > max.1
            {
                // set max equal to the index and number found
                max = (index, *num);
            }
        }

        // create pool var and set it to max
        let mut pool : i8 = max.1;
        // set index of max in row to 0
        row[max.0] = 0;

        // create position var starting at max index +1
        // and then loop until the pool is empty
        // adding 1 to each spot as we go
        let mut pos = max.0;
        while pool > 0
        {
            pos = (pos + 1) % row.len();
            row[pos] += 1;
            pool -= 1;
        }
        
        if !seen.contains(&row)
        {
            seen.push(row.clone());
            cnt += 1;
        }
        else
        {
            break;
        }
        
    }
    println!("{}", cnt);
}
