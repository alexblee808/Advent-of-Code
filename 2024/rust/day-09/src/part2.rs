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
    while position_end >= position_front
        && index_front < values.len()
    {
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

        // Remove free space from the right
        if !!!is_file(index_end) {
            position_end -= values[index_end];
            values[index_end] = 0;
        }

        // Allocate the file from the right
        let mut inner_index_front: usize = index_front;
        while inner_index_front < index_end {
            if is_file(inner_index_front)
                || values[inner_index_front]
                    < values[index_end]
            {
                inner_index_front += 1;
            }
            for _ in 0..values[index_end] {
                checksum +=
                    (position_front * id_number_end) as u64;
                position_front += 1;
            }
            values[inner_index_front] -= values[index_end];
            values[index_end] = 0;
            break;
        }

        // Allocate the file inplace because it cannot be
        // moved
        if values[index_end] != 0 {
            println!("Adding file in place");
        }

        // // Handle case when a file is being
        // allocated from // the left
        // if is_file(index_front) {
        //     println!("{}", id_number_front);
        //     checksum +=
        //         (id_number_front *
        // position_front) as u64;
        //     values[index_front] -= 1;
        //     position_front += 1;
        //     continue;
        // }

        // // Handle case when a file is being
        // allocated from // the right
        // // Skip over the free space
        // if !!!is_file(index_end) {
        //     position_end -= values[index_end];
        //     values[index_end] = 0;
        //     continue;
        // }

        // // Loop over the file IDs in decreasing
        // order
        // let mut inner_index_front: usize =
        // index_front; while index_front
        // < inner_index_end {
        //     if index_front >= inner_index_end {
        //         values[index_front] = 0;
        //         break;
        //     } else if is_file(inner_index_end)
        // && values[index_front]
        //         >= values[inner_index_end]
        //         && values[inner_index_end] > 0
        //     {
        //         let inner_id_number_end =
        //             index_end as u32 / 2;
        //         for _ in
        // 0..values[inner_index_end] {
        //             checksum +=
        // (inner_id_number_end
        //                 * position_front)
        //                 as u64;
        //             position_front += 1;
        //             position_end -= 1;
        //             println!("{}",
        // inner_id_number_end);         }
        //         values[index_front] -=
        //             values[inner_index_end];
        //         values[inner_index_end] = 0;
        //     }
        //     inner_index_end -= 1;
        // }
    }

    Ok(checksum.to_string())
}

fn is_file(disk_index: usize) -> bool {
    disk_index % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
