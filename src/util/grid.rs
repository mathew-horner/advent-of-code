pub type Grid<T> = Vec<Vec<T>>;

#[allow(dead_code)]
pub fn debug_grid<T>(grid: &Grid<T>)
where
    T: std::fmt::Display,
{
    grid.into_iter().for_each(|row| {
        row.into_iter().for_each(|col| print!("{col}"));
        println!();
    });
}
