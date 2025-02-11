#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut values: Vec<u32> = input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();



    let mut value_index: usize = 0;
    let mut memory_index: u32 = 0;
    let mut move_memory_index: u32 =
    values.iter().sum::<u32>() - 1;
    let mut move_file_id: u32 = values.len() as u32 / 2;

    let mut checksum = 0;
    while memory_index < move_memory_index {
        match value_index % 2 { 
            0 => {
                // Normal file, add away
                let file_id = (value_index / 2) as u32;
                match values[value_index] {
                    0 => value_index += 1,
                    _ => checksum += file_id * memory_index
                        
                    
                }

                

            }
            _ => 
        };

        values[value_index] = -1;

    }


























    let mut index: u32 = 0;
    let mut file_id: u32 = 0;

    let mut shift_index: u32 =
        values.iter().sum::<u32>() - 1;
    let mut shift_file_id: u32 = values.len() as u32 / 2;

    dbg!(&values, shift_file_id, shift_index);

    let mut checksum = 0;
    let mut is_file: bool = true;
    while index < shift_index {
        match is_file {
            true => {
                checksum += 
                index += 1;
            }
            false => {}
        }
    }


    



    




    todo!();
    let mut spaces: Vec<i32> = Vec::with_capacity(
        values.iter().sum::<u32>() as usize,
    );

    values.iter().enumerate().for_each(|(i, count)| {
        let value: i32 =
            if 0 == i % 2 { (i / 2) as i32 } else { -1 };
        for _ in 0..*count {
            spaces.push(value);
        }
    });
    dbg!(&spaces);

    let mut checksum = 0;
    let mut index: i32 = 0;
    let mut shift_index: i32 = spaces.len() as i32;

    while index < shift_index {
        match spaces[index as usize] {
            -1 => {
                shift_index += -1;
                match spaces[shift_index as usize] {
                    -1 => continue,
                    _ => {
                        checksum += spaces
                            [shift_index as usize]
                            * index;
                        index += 1;
                    }
                }
            }
            _ => {
                checksum += spaces[index as usize] * index;
                index += 1
            }
        }
        dbg!(checksum);
    }

    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
