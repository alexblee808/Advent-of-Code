use std::collections::HashMap;

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
    // Disk Map Index
    let mut index_front: usize = 0;
    let mut index_end: usize = values.len() - 1;

    let mut checksum: u64 = 0;
    // Use Vec of same size as values initialized to
    // zeroes
    let mut allocated: HashMap<usize, u32> = HashMap::new();
    while position_end >= position_front
        && index_front < values.len()
    {
        // Handle case when a file is being allocated from
        // the left
        if is_file(index_front) {
            // Check if the file has been added from the
            // right
            if allocated.contains_key(&index_front) {
                position_front += allocated[&index_front];
            } else {
                let length: u32 = values[index_front];
                checksum += (id_number_front
                    * (position_front * length
                        + (0..length).sum::<u32>()))
                    as u64;
                // // DELETE ME
                // for _ in 0..length {
                //     dbg!("{}", &id_number_front);
                // }
                // // DELETE ME
                position_front += length;
            }

            values[index_front] = 0;
            index_front += 1;
            id_number_front = index_front as u32 / 2;
            continue;
        }

        // TEST WITH AND WITHOUT
        if allocated.contains_key(&index_end) {
            position_end -= allocated[&index_end]
                + values[index_end - 1];
            index_end -= 2;
        }
        // TEST WITH AND WITHOUT

        // Attempt to allocate file from right
        let mut free_space: u32 = values[index_front];
        let mut index_end_inner: usize = index_end;
        while free_space > 0
            && index_front < index_end_inner
        {
            let length_end: u32 = values[index_end_inner];
            if length_end > free_space || length_end == 0 {
                index_end_inner -= 2;
                continue;
            }
            let id_number_end_inner: u32 =
                index_end_inner as u32 / 2;
            checksum += (id_number_end_inner
                * (position_front * length_end
                    + (0..length_end).sum::<u32>()))
                as u64;
            // // DELETE ME
            // for _ in 0..length_end {
            //     println!("{}", &id_number_end_inner);
            // }
            // // DELETE ME

            allocated.insert(index_end_inner, length_end);
            values[index_end_inner] = 0;
            position_front += length_end;
            free_space -= length_end;
            index_end_inner -= 2;
        }
        if free_space > 0 {
            position_front += free_space;
        }
        values[index_front] = 0;
        index_front += 1;
        id_number_front = index_front as u32 / 2;

        //     // Remove free space from the right
        //     if !!!is_file(index_end) {
        //         position_end -=
        // values[index_end];
        //         values[index_end] = 0;
        //     }

        //     if is_file(index_front) {
        //         let length: u32 =
        // values[index_front];
        //             checksum +=
        //             (id_number_front * length *
        // position_front)                 as u64
        //                 + (0..length).
        //                   sum::<u32>() as u64;

        //         position_front += length;
        //         index_front += 1;
        //         id_number_front = index_front
        // as u32 /    2;

        //      }

        //     (index_front+1..index_end)
        //     for index_end_inner in
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
