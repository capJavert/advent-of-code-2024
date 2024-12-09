use advent_of_code_2024::fetch_input;

#[derive(Debug, PartialEq, Clone, Copy)]
enum BlockKind {
    File,
    Free,
}

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    size: usize,
    kind: BlockKind,
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(9, 2024).expect("failed to fetch input");

    let mut next_block_id = 0;

    let mut blocks = input
        .trim_end()
        .chars()
        .enumerate()
        .fold(vec![], |mut acc, (index, item)| {
            let size = item.to_string().parse().unwrap();
            let block_kind = if (index + 1) % 2 == 0 {
                BlockKind::Free
            } else {
                BlockKind::File
            };

            let block_id = if block_kind == BlockKind::File {
                next_block_id += 1;

                next_block_id - 1
            } else {
                0
            };

            acc.push(Block {
                id: block_id,
                size,
                kind: block_kind,
            });

            acc
        });

    for file in blocks
        .clone()
        .iter()
        .rev()
        .filter(|block| block.kind == BlockKind::File)
    {
        for (first_free_block, free_space) in blocks
            .iter()
            .enumerate()
            .filter(|block| block.1.kind == BlockKind::Free)
        {
            if free_space.size < file.size {
                continue;
            }

            let last_file_block = blocks.iter().position(|block| block.id == file.id).unwrap();

            if last_file_block < first_free_block {
                continue;
            }

            let diff = free_space.size - file.size;

            blocks.swap(first_free_block, last_file_block);

            if diff > 0 {
                blocks[last_file_block].size = file.size;

                blocks.insert(
                    first_free_block + 1,
                    Block {
                        id: 0,
                        size: diff,
                        kind: BlockKind::Free,
                    },
                );
            }

            break;
        }
    }

    let mut block_index = 0;

    let checksum = blocks.into_iter().fold(0, |mut acc, block| {
        for _ in 0..block.size {
            if block.kind == BlockKind::File {
                acc += block.id * block_index;
            }

            block_index += 1;
        }

        acc
    });

    println!("{}", checksum);

    Ok(())
}
