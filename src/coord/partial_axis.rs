use super::{AsRangedCoord, DiscreteRanged, Ranged};
use std::ops::Range;

/// This axis decorator will make the axis partially display on the axis.
/// At some time, we want the axis only covers some part of the value.
/// This decorator will have an additional display range defined.
pub struct PartialAxis<R: Ranged>(R, Range<R::ValueType>);

/// The trait for the types that can be converted into a partial axis
pub trait IntoPartialAxis: AsRangedCoord {
    /// Make the partial axis
    ///
    /// - `axis_range`: The range of the axis to be displayed
    /// - **returns**: The converted range specification
    fn partial_axis(
        self,
        axis_range: Range<<Self::CoordDescType as Ranged>::ValueType>,
    ) -> PartialAxis<Self::CoordDescType> {
        PartialAxis(self.into(), axis_range)
    }
}

impl<R: AsRangedCoord> IntoPartialAxis for R {}

impl<R: Ranged + Clone> Clone for PartialAxis<R>
where
    <R as Ranged>::ValueType: Clone,
{
    fn clone(&self) -> Self {
        PartialAxis(self.0.clone(), self.1.clone())
    }
}

impl<R: Ranged> Ranged for PartialAxis<R>
where
    R::ValueType: Clone,
{
    type ValueType = R::ValueType;

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        self.0.map(value, limit)
    }

    fn key_points(&self, max_points: usize) -> Vec<Self::ValueType> {
        self.0.key_points(max_points)
    }

    fn range(&self) -> Range<Self::ValueType> {
        self.0.range()
    }

    fn axis_pixel_range(&self, limit: (i32, i32)) -> Range<i32> {
        let left = self.map(&self.1.start, limit);
        let right = self.map(&self.1.end, limit);

        left.min(right)..left.max(right)
    }
}

impl<R: DiscreteRanged> DiscreteRanged for PartialAxis<R>
where
    R: Ranged,
    <R as Ranged>::ValueType: Eq + Clone,
{
    fn size(&self) -> usize {
        self.0.size()
    }

    fn index_of(&self, value: &R::ValueType) -> Option<usize> {
        self.0.index_of(value)
    }

    fn from_index(&self, index: usize) -> Option<Self::ValueType> {
        self.0.from_index(index)
    }
}

/// Make a partial axis based on the percentage of visible portion.
/// We can use `into_partial_axis` to create a partial axis range specification.
/// But sometimes, we want to directly specify the percentage visible to the user.
///
/// - `axis_range`: The range specification
/// - `part`: The visible part of the axis. Each value is from [0.0, 1.0]
/// - **returns**: The partial axis created from the input, or `None` when not possible
pub fn make_partial_axis<T>(
    axis_range: Range<T>,
    part: Range<f64>,
) -> Option<PartialAxis<<Range<T> as AsRangedCoord>::CoordDescType>>
where
    Range<T>: AsRangedCoord,
    T: num_traits::NumCast + Clone,
{
    let left: f64 = num_traits::cast(axis_range.start.clone())?;
    let right: f64 = num_traits::cast(axis_range.end.clone())?;

    let full_range_size = (right - left) / (part.end - part.start);

    let full_left = left - full_range_size * part.start;
    let full_right = right + full_range_size * (1.0 - part.end);

    let full_range: Range<T> = num_traits::cast(full_left)?..num_traits::cast(full_right)?;

    let axis_range: <Range<T> as AsRangedCoord>::CoordDescType = axis_range.into();

    Some(PartialAxis(full_range.into(), axis_range.range()))
}
