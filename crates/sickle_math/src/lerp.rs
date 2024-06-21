use bevy::{ color::Color, ui::{ UiRect, Val } };

pub trait Lerp {
    fn lerp(&self, to: Self, t: f32) -> Self;
}

pub trait Lerp64 {
    fn lerp_64(&self, to: Self, t: f64) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, to: Self, t: f32) -> Self {
        self + (to - self) * t
    }
}

impl Lerp64 for f32 {
    fn lerp_64(&self, to: Self, t: f64) -> Self {
        self + (to - self) * (t as f32)
    }
}

impl Lerp64 for f64 {
    fn lerp_64(&self, to: Self, t: f64) -> Self {
        self + (to - self) * t
    }
}

impl Lerp for Color {
    fn lerp(&self, to: Self, t: f32) -> Self {
        let from = self.linear();
        let target = to.linear();
        Color::srgba(
            from.red.lerp(target.red, t).clamp(0.0, 1.0),
            from.green.lerp(target.green, t).clamp(0.0, 1.0),
            from.blue.lerp(target.blue, t).clamp(0.0, 1.0),
            from.alpha.lerp(target.alpha, t).clamp(0.0, 1.0)
        )
    }
}

impl Lerp for Val {
    fn lerp(&self, to: Self, t: f32) -> Self {
        // We can only LERP between values with the same scale
        match self {
            Val::Auto => self.clone(),
            Val::Px(value) => {
                if let Val::Px(other) = to { Self::Px(value.lerp(other, t)) } else { self.clone() }
            }
            Val::Percent(value) => {
                if let Val::Percent(other) = to {
                    Self::Percent(value.lerp(other, t))
                } else {
                    self.clone()
                }
            }
            Val::Vw(value) => {
                if let Val::Vw(other) = to { Self::Vw(value.lerp(other, t)) } else { self.clone() }
            }
            Val::Vh(value) => {
                if let Val::Vh(other) = to { Self::Vh(value.lerp(other, t)) } else { self.clone() }
            }
            Val::VMin(value) => {
                if let Val::VMin(other) = to {
                    Self::VMin(value.lerp(other, t))
                } else {
                    self.clone()
                }
            }
            Val::VMax(value) => {
                if let Val::VMax(other) = to {
                    Self::VMax(value.lerp(other, t))
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl Lerp for UiRect {
    fn lerp(&self, to: Self, t: f32) -> Self {
        Self::new(
            self.left.lerp(to.left, t),
            self.right.lerp(to.right, t),
            self.top.lerp(to.top, t),
            self.bottom.lerp(to.bottom, t)
        )
    }
}
