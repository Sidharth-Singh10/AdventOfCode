use crate::helpers::file_to_string;

pub fn day5() -> Result<(), Box<dyn std::error::Error>>
{

    let order_string = file_to_string("src/inputs/day5a.txt")?;

    let mut order_vec: Vec<Vec<i32>> = vec![];

    for line in order_string.lines()
    {
        let something:Vec<i32> = line.split("|").map(|s| (s.trim().parse::<i32>().unwrap())).collect();

        let mut found = false;
        for   xyz in  &mut order_vec{

            if something[0]== xyz[0]
            {
                found = true;
                xyz.push(something[1]);
            }

        }
        if !found
        {
            order_vec.push(something);
        }

    }    

    println!("{:?}", order_vec);

    let update_string = file_to_string("src/inputs/day5b.txt")?;
    let mut sum  =0 ;
    for line in update_string.lines()
    {
      let k:Vec<i32> =   line.split(",").map(|s| {s.to_string().trim().parse::<i32>().unwrap()}).collect();
    
     let mut found = false;
    

      for l in   0..k.len()
      {
        
        for xyz in &order_vec{

            if k[l] == xyz[0]
            {
                for i in 1..xyz.len(){

                    for j in 0..l
                    {
                        if xyz[i]==k[j]
                        {
                            found = true;
                            break;
                        }
                    }

                }

            }
        }


      }

      if !found{
        sum += k[k.len()/2];
      }

    

    }

    println!("{}", sum);




    Ok(())

}