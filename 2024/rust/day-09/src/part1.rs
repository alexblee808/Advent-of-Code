#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut values: Vec<u32> = input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Disk Space Pointers
    let mut position_front: u32 = 0;
    let mut position_end: u32 =
        values.iter().sum::<u32>() - 1;
    // ID Number
    let mut id_number_front: u32 = 0;
    let mut id_number_end: u32 =
        (values.len() / 2).try_into().unwrap();
    // Disk Map Index
    let mut index_front: usize = 0;
    let mut index_end: usize = values.len() - 1;

    let mut checksum: u64 = 0;
    while position_end >= position_front {
        // Increment the disk map entries being processed
        if values[index_front] == 0 {
            index_front += 1;
            id_number_front = index_front as u32 / 2;
            continue;
        }
        if values[index_end] == 0 {
            index_end -= 1;
            id_number_end = index_end as u32 / 2;
            continue;
        }

        // Handle case when a file is being allocated from
        // the left
        if index_front % 2 == 0 {
            checksum +=
                (id_number_front * position_front) as u64;
            values[index_front] -= 1;
            position_front += 1;
            continue;
        }

        // Handle case when a file is being allocated from
        // the right
        // Skip over the free space
        if index_end % 2 == 1 {
            position_end -= values[index_end];
            values[index_end] = 0;
            continue;
        }
        checksum += (id_number_end * position_front) as u64;
        values[index_end] -= 1;
        values[index_front] -= 1;
        position_front += 1;
        position_end -= 1;
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
