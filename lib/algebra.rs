pub trait Semigroup {
    type Item;
    fn operate(x: &Self::Item, y: &Self::Item) -> Self::Item;
}

pub trait Moniod: Semigroup {
    fn unit() -> Self::Item;
}

pub trait Group: Moniod {
    fn inverse(x: &Self::Item) -> Self::Item;
    fn sub(x: &Self::Item, y: &Self::Item) -> Self::Item {
        Self::operate(x, &Self::inverse(y))
    }
}
