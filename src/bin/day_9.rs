use advent_of_code_2024::fetch_input;

#[derive(Debug, PartialEq, Clone, Copy)]
enum BlockKind {
    File,
    Free,
}

#[derive(Debug)]
struct Block {
    id: usize,
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

            for _ in 0..size {
                acc.push(Block {
                    id: block_id,
                    kind: block_kind,
                });
            }

            acc
        });

    loop {
        let first_free_block = blocks
            .iter()
            .position(|block| block.kind == BlockKind::Free)
            .unwrap();
        let last_file_block = blocks
            .iter()
            .rposition(|block| block.kind == BlockKind::File)
            .unwrap();

        if last_file_block + 1 == first_free_block {
            break;
        }

        blocks.swap(first_free_block, last_file_block);
    }

    let checksum = blocks
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, block)| {
            if block.kind == BlockKind::File {
                acc + block.id * index
            } else {
                acc
            }
        });

    println!("{}", checksum);

    Ok(())
}
