#[derive(Debug, Clone)]
pub struct PointMixup<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1: Clone, Y1> PointMixup<X1, Y1> {
    pub fn new(
        x: X1,
        y: Y1
    ) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn mixup<X2, Y2>(
        &self,
        other_point: PointMixup<X2, Y2>
    ) -> PointMixup<X1, Y2> {
        PointMixup {
            x: self.x.clone(),
            y: other_point.y,
        }
    }
}