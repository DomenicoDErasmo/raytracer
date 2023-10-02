#[derive(strum::EnumIter)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl TryFrom<i32> for Axis {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Axis::X as i32 => Ok(Axis::X),
            x if x == Axis::Y as i32 => Ok(Axis::Y),
            x if x == Axis::Z as i32 => Ok(Axis::Z),
            _ => Err(()),
        }
    }
}

pub trait AxisIndex<T>
{
    fn axis(&self, axis: &Axis) -> T;
}