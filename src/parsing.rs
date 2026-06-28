pub fn parse_chars(s: &str) -> Result<Vec<Vec<bool>>, String> {
    let grid: Vec<Vec<bool>> = s
        .lines()
        .map(|line| line.chars().map(|ch| ch == 'X').collect())
        .collect();
    Ok(grid)
}
