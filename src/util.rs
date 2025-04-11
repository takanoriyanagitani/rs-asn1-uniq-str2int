#[macro_export]
macro_rules! bind {
    ($io: expr, $mapper: expr) => {
        || {
            let t = $io()?;
            $mapper(t)()
        }
    };
}

#[macro_export]
macro_rules! lift {
    ($pure: expr) => {
        |t| || $pure(t)
    };
}
