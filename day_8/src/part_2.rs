use crate::grid::Grid;

fn visible_count(vantage_height: i32, heights: &[i32]) -> usize {
    for (i, &height) in heights.iter().enumerate() {
        if height >= vantage_height {
            return i + 1;
        }
    }

    heights.len()
}

pub fn part_2(tree_height_grid: &Grid<i32>) {
    let max_score: usize = (0..tree_height_grid.width)
        .flat_map(|col| {
            (0..tree_height_grid.height).map(move |row| {
                let vantage_height = tree_height_grid.get(row, col);

                [
                    tree_height_grid.items_looking_north(row, col),
                    tree_height_grid.items_looking_south(row, col),
                    tree_height_grid.items_looking_east(row, col),
                    tree_height_grid.items_looking_west(row, col),
                ]
                .iter()
                .map(|heights| visible_count(vantage_height, heights))
                .reduce(|a, b| a * b)
                .unwrap()
            })
        })
        .max()
        .unwrap();

    println!("Part 2: {max_score:?}");
}
