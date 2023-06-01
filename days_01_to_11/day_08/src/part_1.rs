use crate::grid::Grid;

pub fn part_1(tree_height_grid: &Grid<i32>) {
    let mut visibility_grid = Grid::new(
        vec![false; tree_height_grid.elements.len()],
        tree_height_grid.width,
    );

    tree_height_grid
        .rows()
        .iter()
        .enumerate()
        .for_each(|(i, tree_height_row)| {
            let visibilities = tree_heights_to_visibilities(tree_height_row);
            visibility_grid.union_row(i, &visibilities);
            let mut tree_height_row = tree_height_row.clone();
            tree_height_row.reverse();
            let mut visibilities = tree_heights_to_visibilities(&tree_height_row);
            visibilities.reverse();
            visibility_grid.union_row(i, &visibilities);
        });

    tree_height_grid
        .columns()
        .iter()
        .enumerate()
        .for_each(|(i, tree_height_row)| {
            let visibilities = tree_heights_to_visibilities(tree_height_row);
            visibility_grid.union_column(i, &visibilities);
            let mut tree_height_row = tree_height_row.clone();
            tree_height_row.reverse();
            let mut visibilities = tree_heights_to_visibilities(&tree_height_row);
            visibilities.reverse();
            visibility_grid.union_column(i, &visibilities);
        });

    let visible_count = visibility_grid
        .elements
        .iter()
        .fold(0, |agg, &b| if b { agg + 1 } else { agg });

    println!("{visibility_grid:?}");
    println!();
    println!("Part 1: {visible_count:?}");
}

fn tree_heights_to_visibilities(tree_heights: &[i32]) -> Vec<bool> {
    let mut max = -1;

    tree_heights
        .iter()
        .map(|&height| {
            if height > max {
                max = height;
                true
            } else {
                false
            }
        })
        .collect()
}
