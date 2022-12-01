#[derive(Debug, Eq)]
pub(crate) struct Elf {
    pub(crate) items: Vec<i64>,
    pub(crate) sum: i64,
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum.cmp(&other.sum)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.sum.partial_cmp(&other.sum)
    }
}

impl Elf {
    pub(crate) fn new(items: Vec<i64>) -> Self {
        Self {
            sum: items.iter().sum(),
            items,
        }
    }

    pub(crate) fn sum(&self) -> i64 {
        self.sum
    }
}

impl From<&str> for Elf {
    fn from(value: &str) -> Self {
        let items: Vec<_> = value
            .lines()
            .map(|l| l.parse().unwrap_or_default())
            .collect();

        Self::new(items)
    }
}
