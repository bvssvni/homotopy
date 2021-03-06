//! A library for homotopy logic.

#![deny(missing_docs)]

use std::ops::{Add, Sub, Mul};
use std::marker::PhantomData;

pub use sides::*;
pub use compose::*;

mod sides;
mod compose;

/// A continuous map between two functions.
pub trait Homotopy<X, Scalar=f64>: Sized {
    /// The output type.
    type Y;

    /// The function being mapped from.
    fn f(&self, X) -> Self::Y;
    /// The function being mapped to.
    fn g(&self, X) -> Self::Y;
    /// A continuous map such that `h(x, 0.0) == f(x)` and `h(x, 1.0) == g(x)`.
    fn h(&self, X, Scalar) -> Self::Y;

    /// Call `h` with default value for `X`.
    ///
    /// This is often used by maps where `X` is a complex unit type, e.g. `((), ())`.
    fn hu(&self, s: Scalar) -> Self::Y where X: Default {self.h(Default::default(), s)}

    /// Gets the inverse.
    fn inverse<'a>(&'a self) -> Inverse<&'a Self> {Inverse(self)}

    /// Gets the diagonal.
    fn diagonal<'a>(&'a self) -> Diagonal<&'a Self, Scalar>
        where Diagonal<&'a Self, Scalar>: Homotopy<X>
    {
        Diagonal::new(self)
    }

    /// Gets the diagonal.
    fn into_diagonal(self) -> Diagonal<Self, Scalar>
        where Diagonal<Self, Scalar>: Homotopy<X>
    {
        Diagonal::new(self)
    }

    /// Gets the left side.
    fn left<'a, S>(&'a self) -> Left<&'a Self>
        where Left<&'a Self>: Homotopy<X, S>
    {
        Left(self)
    }

    /// Gets the left side.
    fn into_left<S>(self) -> Left<Self>
        where Left<Self>: Homotopy<X, S>
    {
        Left(self)
    }

    /// Gets the right side.
    fn right<'a, S>(&'a self) -> Right<&'a Self>
        where Right<&'a Self>: Homotopy<X, S>
    {
        Right(self)
    }

    /// Gets the right side.
    fn into_right<S>(self) -> Right<Self>
        where Right<Self>: Homotopy<X, S>
    {
        Right(self)
    }

    /// Gets the top side.
    fn top<'a, S>(&'a self) -> Top<&'a Self>
        where Top<&'a Self>: Homotopy<X, S>
    {
        Top(self)
    }

    /// Gets the top side.
    fn into_top<S>(self) -> Top<Self>
        where Top<Self>: Homotopy<X, S>
    {
        Top(self)
    }

    /// Gets the bottom side.
    fn bottom<'a, S>(&'a self) -> Bottom<&'a Self>
        where Bottom<&'a Self>: Homotopy<X, S>
    {
        Bottom(self)
    }

    /// Gets the bottom side.
    fn into_bottom<S>(self) -> Bottom<Self>
        where Bottom<Self>: Homotopy<X, S>
    {
        Bottom(self)
    }

    /// Gets the front side.
    fn front<'a, S>(&'a self) -> Front<&'a Self>
        where Front<&'a Self>: Homotopy<X, S>
    {
        Front(self)
    }

    /// Gets the front side.
    fn into_front<S>(self) -> Front<Self>
        where Front<Self>: Homotopy<X, S>
    {
        Front(self)
    }

    /// Gets the back side.
    fn back<'a, S>(&'a self) -> Back<&'a Self>
        where Back<&'a Self>: Homotopy<X, S>
    {
        Back(self)
    }

    /// Gets the back side.
    fn into_back<S>(self) -> Back<Self>
        where Back<Self>: Homotopy<X, S>
    {
        Back(self)
    }

    /// Gets the past side.
    ///
    /// This is for 4D homotopy maps.
    fn past<'a, S>(&'a self) -> Past<&'a Self>
        where Past<&'a Self>: Homotopy<X, S>
    {
        Past(self)
    }

    /// Gets the past side.
    ///
    /// This is for 4D homotopy maps.
    fn into_past<S>(self) -> Past<Self>
        where Past<Self>: Homotopy<X, S>
    {
        Past(self)
    }

    /// Gets the future side.
    ///
    /// This is for 4D homotopy maps.
    fn future<'a, S>(&'a self) -> Future<&'a Self>
        where Future<&'a Self>: Homotopy<X, S>
    {
        Future(self)
    }

    /// Gets the future side.
    ///
    /// This is for 4D homotopy maps.
    fn into_future<S>(self) -> Future<Self>
        where Future<Self>: Homotopy<X, S>
    {
        Future(self)
    }

    /// Gets a left-right intersection, controlled by `s`.
    fn left_right<'a, S>(&'a self, s: f64) -> LeftRight<&'a Self>
        where LeftRight<&'a Self>: Homotopy<X, S>
    {
        LeftRight(self, s)
    }

    /// Gets a left-right intersection, controlled by `s`.
    fn into_left_right<S>(self, s: f64) -> LeftRight<Self>
        where LeftRight<Self>: Homotopy<X, S>
    {
        LeftRight(self, s)
    }

    /// Gets a top-bottom intersection, controlled by `s`.
    fn top_bottom<'a, S>(&'a self, s: f64) -> TopBottom<&'a Self>
        where TopBottom<&'a Self>: Homotopy<X, S>
    {
        TopBottom(self, s)
    }

    /// Gets a top-bottom intersection, controlled by `s`.
    fn into_top_bottom<S>(self, s: f64) -> TopBottom<Self>
        where TopBottom<Self>: Homotopy<X, S>
    {
        TopBottom(self, s)
    }

    /// Gets a front-back intersection, controlled by `s`.
    fn front_back<'a, S>(&'a self, s: f64) -> FrontBack<&'a Self>
        where FrontBack<&'a Self>: Homotopy<X, S>
    {
        FrontBack(self, s)
    }

    /// Gets a front-back intersection, controlled by `s`.
    fn into_front_back<S>(self, s: f64) -> FrontBack<Self>
        where FrontBack<Self>: Homotopy<X, S>
    {
        FrontBack(self, s)
    }

    /// Gets a past-future intersection, controlled by `s`.
    fn past_future<'a, S>(&'a self, s: f64) -> PastFuture<&'a Self>
        where PastFuture<&'a Self>: Homotopy<X, S>
    {
        PastFuture(self, s)
    }

    /// Gets a past-future intersection, controlled by `s`.
    fn into_past_future<S>(self, s: f64) -> PastFuture<Self>
        where PastFuture<Self>: Homotopy<X, S>
    {
        PastFuture(self, s)
    }

    /// Gets a converter to and from vectors.
    fn as_vec<'a, S, VX>(&'a self) -> AsVec<&'a Self>
        where AsVec<&'a Self>: Homotopy<VX, S>
    {
        AsVec(self)
    }

    /// Gets a converter to and from vectors.
    fn into_as_vec<S, VX>(self) -> AsVec<Self>
        where AsVec<Self>: Homotopy<VX, S>
    {
        AsVec(self)
    }

    /// Maps output from one to another.
    fn map<'a, F: Fn(Self::Y) -> Y2, Y2>(&'a self, f: F) -> Map<&'a Self, F, Self::Y, Y2>
        where Map<&'a Self, F, Self::Y, Y2>: Homotopy<X, Scalar>
    {
        Map::new(self, f)
    }

    /// Maps output from one to another.
    fn into_map<F: Fn(Self::Y) -> Y2, Y2>(self, f: F) -> Map<Self, F, Self::Y, Y2>
        where Map<Self, F, Self::Y, Y2>: Homotopy<X, Scalar>
    {
        Map::new(self, f)
    }

    /// Maps output from one to another, into a N+1 homotopy.
    fn smap<'a, F: Fn(Self::Y, f64) -> Y2, Y2>(&'a self, f: F)
    -> SMap<&'a Self, F, Self::Y, Y2, f64>
    {
        SMap::new(self, f)
    }

    /// Maps output from one to another, into a N+1 homotopy.
    fn into_smap<F: Fn(Self::Y, f64) -> Y2, Y2>(self, f: F)
    -> SMap<Self, F, Self::Y, Y2, f64>
    {
        SMap::new(self, f)
    }
}

impl<'a, X, T, S> Homotopy<X, S> for &'a T
    where T: Homotopy<X, S>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {T::f(self, x)}
    fn g(&self, x: X) -> Self::Y {T::g(self, x)}
    fn h(&self, x: X, s: S) -> Self::Y {T::h(self, x, s)}
}

/// Checks that the homotopy constraints hold for some input `x`.
#[must_use]
pub fn check<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X>,
          H::Y: PartialEq,
          X: Clone
{
    h.h(x.clone(), 0.0) == h.f(x.clone()) &&
    h.h(x.clone(), 1.0) == h.g(x)
}

/// Checks that the homotopy constraints hold for default input.
#[must_use]
pub fn checku<H, X>(h: &H) -> bool
    where H: Homotopy<X>,
          H::Y: PartialEq,
          X: Default
{
    h.hu(0.0) == h.f(Default::default()) &&
    h.hu(1.0) == h.g(Default::default())
}

/// Checks that the 2D homotopy constraints hold for some input `x`.
#[must_use]
pub fn check2<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X, [f64; 2]>,
          H::Y: PartialEq,
          X: Clone,
{
    let a = h.f(x.clone());
    let b = h.g(x.clone());
    h.h(x.clone(), [0.0, 0.0]) == a &&
    h.h(x.clone(), [1.0, 1.0]) == b &&
    check(&h.left(), x.clone()) &&
    check(&h.right(), x.clone()) &&
    check(&h.top(), x.clone()) &&
    check(&h.bottom(), x.clone())
}

/// Checks that the 2D homotopy constraints hold for default input.
#[must_use]
pub fn checku2<H, X>(h: &H) -> bool
    where H: Homotopy<X, [f64; 2]>,
          H::Y: PartialEq,
          X: Default,
{
    let a = h.f(Default::default());
    let b = h.g(Default::default());
    h.hu([0.0, 0.0]) == a &&
    h.hu([1.0, 1.0]) == b &&
    checku(&h.left()) &&
    checku(&h.right()) &&
    checku(&h.top()) &&
    checku(&h.bottom())
}

/// Checks that the 3D homotopy constraints hold for some input `x`.
#[must_use]
pub fn check3<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X, [f64; 3]>,
          H::Y: PartialEq,
          X: Clone,
{
    let a = h.f(x.clone());
    let b = h.g(x.clone());
    h.h(x.clone(), [0.0, 0.0, 0.0]) == a &&
    h.h(x.clone(), [1.0, 1.0, 1.0]) == b &&
    check2(&h.left(), x.clone()) &&
    check2(&h.right(), x.clone()) &&
    check2(&h.top(), x.clone()) &&
    check2(&h.bottom(), x.clone()) &&
    check2(&h.front(), x.clone()) &&
    check2(&h.back(), x.clone())
}

/// Checks that the 3D homotopy constraints hold for default input.
#[must_use]
pub fn checku3<H, X>(h: &H) -> bool
    where H: Homotopy<X, [f64; 3]>,
          H::Y: PartialEq,
          X: Default,
{
    let a = h.f(Default::default());
    let b = h.g(Default::default());
    h.hu([0.0, 0.0, 0.0]) == a &&
    h.hu([1.0, 1.0, 1.0]) == b &&
    checku2(&h.left()) &&
    checku2(&h.right()) &&
    checku2(&h.top()) &&
    checku2(&h.bottom()) &&
    checku2(&h.front()) &&
    checku2(&h.back())
}

/// Checks that the 4D homotopy constraints hold for some input `x`.
#[must_use]
pub fn check4<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X, [f64; 4]>,
          H::Y: PartialEq,
          X: Clone,
{
    let a = h.f(x.clone());
    let b = h.g(x.clone());
    h.h(x.clone(), [0.0; 4]) == a &&
    h.h(x.clone(), [1.0; 4]) == b &&
    check3(&h.left(), x.clone()) &&
    check3(&h.right(), x.clone()) &&
    check3(&h.top(), x.clone()) &&
    check3(&h.bottom(), x.clone()) &&
    check3(&h.front(), x.clone()) &&
    check3(&h.back(), x.clone()) &&
    check3(&h.past(), x.clone()) &&
    check3(&h.future(), x.clone())
}

/// Checks that the 4D homotopy constraints hold for default input.
#[must_use]
pub fn checku4<H, X>(h: &H) -> bool
    where H: Homotopy<X, [f64; 4]>,
          H::Y: PartialEq,
          X: Default,
{
    let a = h.f(Default::default());
    let b = h.g(Default::default());
    h.hu([0.0; 4]) == a &&
    h.hu([1.0; 4]) == b &&
    checku3(&h.left()) &&
    checku3(&h.right()) &&
    checku3(&h.top()) &&
    checku3(&h.bottom()) &&
    checku3(&h.front()) &&
    checku3(&h.back()) &&
    checku3(&h.past()) &&
    checku3(&h.future())
}

/// Identity homotopy.
///
/// `f`, `g` and `h` uses the identity function, so this is a homotopy.
#[derive(Copy, Clone)]
pub struct Id;

impl<X, S> Homotopy<X, S> for Id {
    type Y = X;

    fn f(&self, x: X) -> X {x}
    fn g(&self, x: X) -> X {x}
    fn h(&self, x: X, _: S) -> X {x}
}

/// The Dirac function.
#[derive(Copy, Clone)]
pub struct Dirac;

impl Homotopy<()> for Dirac {
    type Y = f64;

    fn f(&self, _: ()) -> f64 {1.0}
    fn g(&self, _: ()) -> f64 {0.0}
    fn h(&self, _: (), s: f64) -> f64 {if s == 0.0 {1.0} else {0.0}}
}

/// Dirac From homotopy.
///
/// Define `h` to be `f` at 0.0 and `g` elsewhere.
/// Since `h` is `g` at 1.0, this is a homotopy.
#[derive(Copy, Clone)]
pub struct DiracFrom<X, Y, F, G>
    where F: Fn(X) -> Y, G: Fn(X) -> Y
{
    fx: F,
    gx: G,
    _x: PhantomData<X>,
    _y: PhantomData<Y>,
}

impl<X, Y, F, G> DiracFrom<X, Y, F, G>
    where F: Fn(X) -> Y, G: Fn(X) -> Y
{
    /// Creates a new `DiracFrom`.
    pub fn new(f: F, g: G) -> DiracFrom<X, Y, F, G> {
        DiracFrom {
            fx: f,
            gx: g,
            _x: PhantomData,
            _y: PhantomData,
        }
    }
}

impl<X, Y, F, G> Homotopy<X> for DiracFrom<X, Y, F, G>
    where Y: Clone,
          F: Fn(X) -> Y,
          G: Fn(X) -> Y
{
    type Y = Y;

    fn f(&self, x: X) -> Y {(self.fx)(x)}
    fn g(&self, x: X) -> Y {(self.gx)(x)}
    fn h(&self, x: X, s: f64) -> Y {
        if s == 0.0 {(self.fx)(x)}
        else {(self.gx)(x)}
    }
}

/// Linear interpolation homotopy.
///
/// `f` and `g` are functions mapping `()` to a value.
/// The scalar passed to `h` controls the linear map.
#[derive(Copy, Clone)]
pub struct Lerp<X>(pub X, pub X);

impl<Y> Homotopy<()> for Lerp<Y>
    where Y: Mul<f64, Output = Y> + Add<Output = Y> + Clone
{
    type Y = Y;

    fn f(&self, _: ()) -> Y {self.0.clone()}
    fn g(&self, _: ()) -> Y {self.1.clone()}
    fn h(&self, _: (), s: f64) -> Y {self.0.clone() * (1.0 - s) + self.1.clone() * s}
}

/// Quadratic Bezier homotopy.
///
/// Maps from point A to C using a point B as control point.
#[derive(Copy, Clone)]
pub struct QuadraticBezier<X>(pub X, pub X, pub X);

impl<X> QuadraticBezier<X> {
    /// Creates a quadratic bezier that is identical to linear interpolation.
    pub fn from_linear(a: X, b: X) -> QuadraticBezier<X>
        where X: Mul<f64, Output = X> + Add<Output = X> + Clone
    {
        QuadraticBezier(a.clone(), a * 0.5 + b.clone() * 0.5, b)
    }
}

impl<X> From<Lerp<X>> for QuadraticBezier<X>
    where X: Mul<f64, Output = X> + Add<Output = X> + Clone
{
    fn from(lerp: Lerp<X>) -> QuadraticBezier<X> {
        QuadraticBezier::from_linear(lerp.0, lerp.1)
    }
}

impl<Y> Homotopy<()> for QuadraticBezier<Y>
    where Y: Mul<f64, Output = Y> + Add<Output = Y> + Clone
{
    type Y = Y;

    fn f(&self, _: ()) -> Y {self.0.clone()}
    fn g(&self, _: ()) -> Y {self.2.clone()}
    fn h(&self, _: (), s: f64) -> Y {
        let a = Lerp(self.0.clone(), self.1.clone()).h((), s);
        let b = Lerp(self.1.clone(), self.2.clone()).h((), s);
        Lerp(a, b).h((), s)
    }
}

/// Cubic Bezier homotopy.
///
/// Maps from point A to D using point B and C as control points.
#[derive(Copy, Clone)]
pub struct CubicBezier<X>(pub X, pub X, pub X, pub X);

impl<X> CubicBezier<X> {
    /// Creates a cubic bezier that is identical to quadratic bezier.
    pub fn from_quadratic(a: X, b: X, c: X) -> CubicBezier<X>
        where X: Clone
    {
        CubicBezier(a, b.clone(), b, c)
    }
}

impl<X> From<QuadraticBezier<X>> for CubicBezier<X>
    where X: Clone
{
    fn from(QuadraticBezier(a, b, c): QuadraticBezier<X>) -> CubicBezier<X> {
        CubicBezier::from_quadratic(a, b, c)
    }
}

impl<Y> Homotopy<()> for CubicBezier<Y>
    where Y: Mul<f64, Output = Y> + Add<Output = Y> + Clone
{
    type Y = Y;

    fn f(&self, _: ()) -> Y {self.0.clone()}
    fn g(&self, _: ()) -> Y {self.3.clone()}
    fn h(&self, _: (), s: f64) -> Y {
        let a = Lerp(self.0.clone(), self.1.clone()).h((), s);
        let b = Lerp(self.2.clone(), self.3.clone()).h((), s);
        Lerp(a, b).h((), s)
    }
}

/// Takes the square of two homotopy maps and produces a 2D homotopy.
#[derive(Copy, Clone)]
pub struct Square<X1, X2, H1, H2>
    where H1: Homotopy<X1>, H2: Homotopy<X2>
{
    h1: H1,
    h2: H2,
    _x1: PhantomData<X1>,
    _x2: PhantomData<X2>,
}

impl<X1, X2, H1, H2> Square<X1, X2, H1, H2>
    where H1: Homotopy<X1>, H2: Homotopy<X2>
{
    /// Creates a square of two homotopy maps.
    pub fn new(h1: H1, h2: H2) -> Self {
        Square {h1, h2, _x1: PhantomData, _x2: PhantomData}
    }
}

impl<X1, X2, H1, H2> Homotopy<(X1, X2), [f64; 2]> for Square<X1, X2, H1, H2>
    where H1: Homotopy<X1>, H2: Homotopy<X2>
{
    type Y = (H1::Y, H2::Y);

    fn f(&self, x: (X1, X2)) -> Self::Y {(self.h1.f(x.0), self.h2.f(x.1))}
    fn g(&self, x: (X1, X2)) -> Self::Y {(self.h1.g(x.0), self.h2.g(x.1))}
    fn h(&self, x: (X1, X2), s: [f64; 2]) -> Self::Y {(self.h1.h(x.0, s[0]), self.h2.h(x.1, s[1]))}
}

/// Takes the cube of three homotopy maps and produces a 3D homotopy.
#[derive(Copy, Clone)]
pub struct Cube<X1, X2, X3, H1, H2, H3>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>
{
    h1: H1,
    h2: H2,
    h3: H3,
    _x1: PhantomData<X1>,
    _x2: PhantomData<X2>,
    _x3: PhantomData<X3>,
}

impl<X1, X2, X3, H1, H2, H3> Cube<X1, X2, X3, H1, H2, H3>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>
{
    /// Creates a new cube of three homotopy maps.
    pub fn new(h1: H1, h2: H2, h3: H3) -> Self {
        Cube {h1, h2, h3, _x1: PhantomData, _x2: PhantomData, _x3: PhantomData}
    }
}

impl<X1, X2, X3, H1, H2, H3> Homotopy<(X1, X2, X3), [f64; 3]> for Cube<X1, X2, X3, H1, H2, H3>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>
{
    type Y = (H1::Y, H2::Y, H3::Y);

    fn f(&self, x: (X1, X2, X3)) -> Self::Y {(self.h1.f(x.0), self.h2.f(x.1), self.h3.f(x.2))}
    fn g(&self, x: (X1, X2, X3)) -> Self::Y {(self.h1.g(x.0), self.h2.g(x.1), self.h3.g(x.2))}
    fn h(&self, x: (X1, X2, X3), s: [f64; 3]) -> Self::Y {
        (self.h1.h(x.0, s[0]), self.h2.h(x.1, s[1]), self.h3.h(x.2, s[2]))
    }
}

/// Takes the 4-cube of four homotopy maps and produces a 4D homotopy.
#[derive(Copy, Clone)]
pub struct Cube4<X1, X2, X3, X4, H1, H2, H3, H4>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>, H4: Homotopy<X4>
{
    h1: H1,
    h2: H2,
    h3: H3,
    h4: H4,
    _x1: PhantomData<X1>,
    _x2: PhantomData<X2>,
    _x3: PhantomData<X3>,
    _x4: PhantomData<X4>,
}

impl<X1, X2, X3, X4, H1, H2, H3, H4> Cube4<X1, X2, X3, X4, H1, H2, H3, H4>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>, H4: Homotopy<X4>
{
    /// Creates a new 4-cube of four homotopy maps.
    pub fn new(h1: H1, h2: H2, h3: H3, h4: H4) -> Self {
        Cube4 {
            h1, h2, h3, h4,
            _x1: PhantomData,
            _x2: PhantomData,
            _x3: PhantomData,
            _x4: PhantomData,
        }
    }
}

impl<X1, X2, X3, X4, H1, H2, H3, H4> Homotopy<(X1, X2, X3, X4), [f64; 4]>
for Cube4<X1, X2, X3, X4, H1, H2, H3, H4>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>, H4: Homotopy<X4>
{
    type Y = (H1::Y, H2::Y, H3::Y, H4::Y);

    fn f(&self, x: (X1, X2, X3, X4)) -> Self::Y {
        (self.h1.f(x.0), self.h2.f(x.1), self.h3.f(x.2), self.h4.f(x.3))
    }
    fn g(&self, x: (X1, X2, X3, X4)) -> Self::Y {
        (self.h1.g(x.0), self.h2.g(x.1), self.h3.g(x.2), self.h4.g(x.3))
    }
    fn h(&self, x: (X1, X2, X3, X4), s: [f64; 4]) -> Self::Y {
        (self.h1.h(x.0, s[0]), self.h2.h(x.1, s[1]), self.h3.h(x.2, s[2]), self.h4.h(x.3, s[3]))
    }
}

/// Inverts the direction of a homotopy.
pub struct Inverse<T>(pub T);

impl<X, T> Homotopy<X> for Inverse<T>
    where T: Homotopy<X>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.g(x)}
    fn g(&self, x: X) -> Self::Y {self.0.f(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, 1.0 - s)}
}

/// Converts to and from vectors.
#[derive(Copy, Clone)]
pub struct AsVec<T>(pub T);

impl<X, Y, S, T> Homotopy<[X; 2], S> for AsVec<T>
    where T: Homotopy<(X, X), S, Y = (Y, Y)>, X: Copy
{
    type Y = [Y; 2];

    fn f(&self, x: [X; 2]) -> Self::Y {
        let (a, b) = self.0.f((x[0], x[1]));
        [a, b]
    }
    fn g(&self, x: [X; 2]) -> Self::Y {
        let (a, b) = self.0.g((x[0], x[1]));
        [a, b]
    }
    fn h(&self, x: [X; 2], s: S) -> Self::Y {
        let (a, b) = self.0.h((x[0], x[1]), s);
        [a, b]
    }
}

impl<X, Y, S, T> Homotopy<[X; 3], S> for AsVec<T>
    where T: Homotopy<(X, X, X), S, Y = (Y, Y, Y)>, X: Copy
{
    type Y = [Y; 3];

    fn f(&self, x: [X; 3]) -> Self::Y {
        let (a, b, c) = self.0.f((x[0], x[1], x[2]));
        [a, b, c]
    }
    fn g(&self, x: [X; 3]) -> Self::Y {
        let (a, b, c) = self.0.g((x[0], x[1], x[2]));
        [a, b, c]
    }
    fn h(&self, x: [X; 3], s: S) -> Self::Y {
        let (a, b, c) = self.0.h((x[0], x[1], x[2]), s);
        [a, b, c]
    }
}

impl<X, Y, S, T> Homotopy<[X; 4], S> for AsVec<T>
    where T: Homotopy<(X, X, X, X), S, Y = (Y, Y, Y, Y)>, X: Copy
{
    type Y = [Y; 4];

    fn f(&self, x: [X; 4]) -> Self::Y {
        let (a, b, c, d) = self.0.f((x[0], x[1], x[2], x[3]));
        [a, b, c, d]
    }
    fn g(&self, x: [X; 4]) -> Self::Y {
        let (a, b, c, d) = self.0.g((x[0], x[1], x[2], x[3]));
        [a, b, c, d]
    }
    fn h(&self, x: [X; 4], s: S) -> Self::Y {
        let (a, b, c, d) = self.0.h((x[0], x[1], x[2], x[3]), s);
        [a, b, c, d]
    }
}

/// Generates points on a circle.
#[derive(Copy, Clone)]
pub struct Circle<T> {
    /// Center of circle.
    pub center: [T; 2],
    /// Radius of circle.
    pub radius: T
}

impl<T> Homotopy<()> for Circle<T>
    where T: Clone + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T>
{
    type Y = [T; 2];

    fn f(&self, _: ()) -> Self::Y {
        [self.center[0].clone() + self.radius.clone(), self.center[1].clone()]
    }
    fn g(&self, _: ()) -> Self::Y {
        [self.center[0].clone() + self.radius.clone(), self.center[1].clone()]
    }
    fn h(&self, _: (), s: f64) -> Self::Y {
        // Handle special cases to get exact values.
        if s == 1.0 {
            return self.g(())
        } else if s == 0.5 {
            return [self.center[0].clone() - self.radius.clone(), self.center[1].clone()]
        } else if s == 0.25 {
            return [self.center[0].clone(), self.center[1].clone() + self.radius.clone()]
        } else if s == 0.75 {
            return [self.center[0].clone(), self.center[1].clone() - self.radius.clone()]
        };
        [
            self.center[0].clone() + self.radius.clone() * (s * std::f64::consts::PI * 2.0).cos(),
            self.center[1].clone() + self.radius.clone() * (s * std::f64::consts::PI * 2.0).sin(),
        ]
    }
}

/// Maps output of homotopy map from one form into another.
#[derive(Clone, Copy)]
pub struct Map<H, F, Y1, Y2>
    where F: Fn(Y1) -> Y2
{
    hom: H,
    fun: F,
    _y1: PhantomData<Y1>,
    _y2: PhantomData<Y2>,
}

impl<H, F, Y1, Y2> Map<H, F, Y1, Y2>
    where F: Fn(Y1) -> Y2
{
    /// Creates new map.
    pub fn new(h: H, f: F) -> Self {
        Map {
            hom: h, fun: f, _y1: PhantomData, _y2: PhantomData
        }
    }
}

impl<H, F, Y2, X, S> Homotopy<X, S> for Map<H, F, H::Y, Y2>
    where H: Homotopy<X, S>, F: Fn(H::Y) -> Y2
{
    type Y = Y2;

    fn f(&self, x: X) -> Self::Y {(self.fun)(self.hom.f(x))}
    fn g(&self, x: X) -> Self::Y {(self.fun)(self.hom.g(x))}
    fn h(&self, x: X, s: S) -> Self::Y {(self.fun)(self.hom.h(x, s))}
}

/// Maps output of an N-homotopy map from one form into an N+1 homotopy.
///
/// This is used when the output contains extra structure you want to interpolate over,
/// or when you want to extend the shape along some new dimension.
#[derive(Clone, Copy)]
pub struct SMap<H, F, Y1, Y2, S>
    where F: Fn(Y1, S) -> Y2
{
    hom: H,
    fun: F,
    _y1: PhantomData<Y1>,
    _y2: PhantomData<Y2>,
    _s : PhantomData<S>,
}

impl<H, F, Y1, Y2, S> SMap<H, F, Y1, Y2, S>
    where F: Fn(Y1, S) -> Y2
{
    /// Creates new map.
    pub fn new(h: H, f: F) -> Self {
        SMap {
            hom: h, fun: f, _y1: PhantomData, _y2: PhantomData, _s: PhantomData
        }
    }
}

impl<H, F, Y2, X> Homotopy<X, [f64; 2]> for SMap<H, F, H::Y, Y2, f64>
    where H: Homotopy<X>, F: Fn(H::Y, f64) -> Y2
{
    type Y = Y2;

    fn f(&self, x: X) -> Self::Y {(self.fun)(self.hom.f(x), 0.0)}
    fn g(&self, x: X) -> Self::Y {(self.fun)(self.hom.g(x), 1.0)}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {(self.fun)(self.hom.h(x, s[0]), s[1])}
}

impl<H, F, Y2, X> Homotopy<X, [f64; 3]> for SMap<H, F, H::Y, Y2, f64>
    where H: Homotopy<X, [f64; 2]>, F: Fn(H::Y, f64) -> Y2
{
    type Y = Y2;

    fn f(&self, x: X) -> Self::Y {(self.fun)(self.hom.f(x), 0.0)}
    fn g(&self, x: X) -> Self::Y {(self.fun)(self.hom.g(x), 1.0)}
    fn h(&self, x: X, s: [f64; 3]) -> Self::Y {(self.fun)(self.hom.h(x, [s[0], s[1]]), s[2])}
}

impl<H, F, Y2, X> Homotopy<X, [f64; 4]> for SMap<H, F, H::Y, Y2, f64>
    where H: Homotopy<X, [f64; 3]>, F: Fn(H::Y, f64) -> Y2
{
    type Y = Y2;

    fn f(&self, x: X) -> Self::Y {(self.fun)(self.hom.f(x), 0.0)}
    fn g(&self, x: X) -> Self::Y {(self.fun)(self.hom.g(x), 1.0)}
    fn h(&self, x: X, s: [f64; 4]) -> Self::Y {(self.fun)(self.hom.h(x, [s[0], s[1], s[2]]), s[3])}
}

impl<T, S> Homotopy<usize, S> for Vec<T>
    where T: Homotopy<(), S>
{
    type Y = T::Y;

    fn f(&self, ind: usize) -> Self::Y {self[ind].f(())}
    fn g(&self, ind: usize) -> Self::Y {self[ind].g(())}
    fn h(&self, ind: usize, s: S) -> Self::Y {self[ind].h((), s)}
}

impl<T, S, X> Homotopy<(usize, X), S> for Vec<T>
    where T: Homotopy<X, S>
{
    type Y = T::Y;

    fn f(&self, (ind, x): (usize, X)) -> Self::Y {self[ind].f(x)}
    fn g(&self, (ind, x): (usize, X)) -> Self::Y {self[ind].g(x)}
    fn h(&self, (ind, x): (usize, X), s: S) -> Self::Y {self[ind].h(x, s)}
}

/// Translate some distance.
#[derive(Copy, Clone)]
pub struct Translate<X>(pub X);

impl Homotopy<f64> for Translate<f64> {
    type Y = f64;

    fn f(&self, x: f64) -> Self::Y {x}
    fn g(&self, x: f64) -> Self::Y {x + self.0}
    fn h(&self, x: f64, s: f64) -> Self::Y {x + s * self.0}
}

impl Homotopy<[f64; 2]> for Translate<[f64; 2]> {
    type Y = [f64; 2];

    fn f(&self, x: [f64; 2]) -> Self::Y {x}
    fn g(&self, x: [f64; 2]) -> Self::Y {[x[0] + self.0[0], x[1] + self.0[1]]}
    fn h(&self, x: [f64; 2], s: f64) -> Self::Y {[x[0] + s * self.0[0], x[1] + s * self.0[1]]}
}

impl Homotopy<[f64; 3]> for Translate<[f64; 3]> {
    type Y = [f64; 3];

    fn f(&self, x: [f64; 3]) -> Self::Y {x}
    fn g(&self, x: [f64; 3]) -> Self::Y {
        [
            x[0] + self.0[0],
            x[1] + self.0[1],
            x[2] + self.0[2],
        ]
    }
    fn h(&self, x: [f64; 3], s: f64) -> Self::Y {
        [
            x[0] + s * self.0[0],
            x[1] + s * self.0[1],
            x[2] + s * self.0[2],
        ]
    }
}

impl Homotopy<[f64; 4]> for Translate<[f64; 4]> {
    type Y = [f64; 4];

    fn f(&self, x: [f64; 4]) -> Self::Y {x}
    fn g(&self, x: [f64; 4]) -> Self::Y {
        [
            x[0] + self.0[0],
            x[1] + self.0[1],
            x[2] + self.0[2],
            x[3] + self.0[3],
        ]
    }
    fn h(&self, x: [f64; 4], s: f64) -> Self::Y {
        [
            x[0] + s * self.0[0],
            x[1] + s * self.0[1],
            x[2] + s * self.0[2],
            x[3] + s * self.0[3],
        ]
    }
}

/// Create a sweep from two circles.
///
/// This is constructed by taking the diagonal of the square product of two circles.
/// It can be thought of as making the circles rotate together, controlled by a single parameter.
///
/// Then, a SMap adds a new dimension that interpolates along the sweep,
/// making it possible to control both the rotation and position between the two circles.
pub fn sweep(a: Circle<f64>, b: Circle<f64>) -> impl Homotopy<((), ()), [f64; 2], Y = [f64; 2]> {
    Square::new(a, b).into_diagonal().into_smap(|(a, b), s| [
            a[0] + (b[0] - a[0]) * s,
            a[1] + (b[1] - a[1]) * s,
        ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_id() {
        assert!(check(&Id, 0.0 as f64));
        assert!(check(&Id, 1.0 as f64));
        assert!(check(&Id, true));
        assert!(check(&Id, false));
    }

    #[test]
    fn check_dirac() {
        assert!(checku(&Dirac));
    }

    #[test]
    fn check_dirac_from() {
        let ft = DiracFrom::new(|()| 1.0, |()| 0.0);
        assert!(checku(&ft));
    }

    #[test]
    fn check_lerp() {
        let lerp = Lerp(1.2, 1.3);
        assert!(checku(&lerp));
    }

    #[test]
    fn check_quadratic_bezier() {
        let qb = QuadraticBezier(0.3, 0.7, 0.9);
        assert!(checku(&qb));
    }

    #[test]
    fn check_cubic_bezier() {
        let cb = CubicBezier(0.3, 0.7, 0.8, 0.9);
        assert!(checku(&cb));
    }

    #[test]
    fn check_reduced_quadratic_bezier_equals_lerp() {
        let qb = QuadraticBezier::from_linear(0.0, 1.0);
        let l = Lerp(0.0, 1.0);
        let mut s = 0.0;
        loop {
            assert!((qb.hu(s) - l.hu(s)).abs() < 0.000001);
            s += 0.1;
            if s > 1.0 {break}
        }
    }

    #[test]
    fn check_reduced_cubic_bezier_equals_quadratic_bezier() {
        let cb = CubicBezier::from_quadratic(0.0, 0.3, 0.9);
        let qb = QuadraticBezier(0.0, 0.3, 0.9);
        let mut s = 0.0;
        loop {
            assert_eq!(cb.hu(s), qb.hu(s));
            s += 0.1;
            if s > 1.0 {break}
        }
    }

    #[test]
    fn check_composition() {
        // Create a linear interpolation.
        let a = Lerp(3.0, 10.0);
        assert_eq!(a.hu(0.0), 3.0);
        assert_eq!(a.hu(0.5), 6.5);
        assert_eq!(a.hu(1.0), 10.0);
        // Compose with a Dirac From that seperates the start of the line
        // from the rest of the line.
        let b = DiracFrom::new(|x| x - 2.0, |x| x + 2.0);
        let c = Compose::new(a, b);
        assert!(checku2(&c));

        assert_eq!(c.hu([0.0, 0.0]), 1.0);
        assert_eq!(c.hu([0.0000000000000001, 0.0]), 1.0000000000000004);
        assert_eq!(c.hu([0.5, 0.0]), 4.5);
        assert_eq!(c.hu([1.0, 0.0]), 8.0);

        assert_eq!(c.hu([0.0, 1.0]), 5.0);
        assert_eq!(c.hu([0.0000000000000001, 1.0]), 5.0);
        assert_eq!(c.hu([0.5, 1.0]), 8.5);
        assert_eq!(c.hu([1.0, 1.0]), 12.0);

        let d = c.diagonal();
        assert_eq!(d.hu(0.0), 1.0);
        assert_eq!(d.hu(0.0000000000000001), 5.0);
        assert_eq!(d.hu(0.5), 8.5);
        assert_eq!(d.hu(1.0), 12.0);
    }

    #[test]
    fn check_square() {
        let a = Lerp(1.0, 5.0);
        let b = Lerp(11.0, 15.0);
        let c = Square::new(a, b);
        assert!(checku2(&c));
        assert!(checku(&c.diagonal()));
        assert!(checku2(&c.as_vec()));
        assert!(checku(&c.left_right(0.5)));
        assert!(checku(&c.top_bottom(0.5)));
    }

    #[test]
    fn check_cube() {
        let a = Lerp(1.0, 2.0);
        let b = Lerp(3.0, 4.0);
        let c = Lerp(5.0, 6.0);
        let c = Cube::new(a, b, c);
        assert!(checku3(&c));
        assert!(checku(&c.diagonal()));
        assert!(checku3(&c.as_vec()));
        assert!(checku2(&c.left_right(0.5)));
        assert!(checku2(&c.top_bottom(0.5)));
        assert!(checku2(&c.front_back(0.5)));
    }

    #[test]
    fn check_cube4() {
        let a = Lerp(1.0, 2.0);
        let b = Lerp(3.0, 4.0);
        let c = Lerp(5.0, 6.0);
        let d = Lerp(7.0, 8.0);
        let c = Cube4::new(a, b, c, d);
        assert!(checku4(&c));
        assert!(checku(&c.diagonal()));
        assert!(checku4(&c.as_vec()));
        assert!(checku3(&c.left_right(0.5)));
        assert!(checku3(&c.top_bottom(0.5)));
        assert!(checku3(&c.front_back(0.5)));
        assert!(checku3(&c.past_future(0.5)));
    }

    #[test]
    fn check_invert() {
        let a = Lerp(2.0, 4.0);
        let b = a.inverse();
        assert!(checku(&b));
    }

    #[test]
    fn check_circle() {
        let a = Circle {center: [0.0, 0.0], radius: 1.0};
        assert!(check(&a, ()));
        assert_eq!(a.hu(0.0), [1.0, 0.0]);
        assert_eq!(a.hu(0.5), [-1.0, 0.0]);
        assert_eq!(a.hu(0.25), [0.0, 1.0]);
        assert_eq!(a.hu(0.75), [0.0, -1.0]);
        assert_eq!(a.hu(1.0), [1.0, 0.0]);

        let b = Circle {center: [0.0, 0.0], radius: 2.0};
        let c = Square::new(a, b);
        let c = c.as_vec();
        assert!(checku2(&c));
        assert_eq!(c.hu([0.5, 0.25]), [[-1.0, 0.0], [0.0, 2.0]]);

        // A diagonal of a square of two circles is a sector sweep.
        let diag_c = c.diagonal();
        assert_eq!(diag_c.hu(0.0), [[1.0, 0.0], [2.0, 0.0]]);
        assert_eq!(diag_c.hu(0.25), [[0.0, 1.0], [0.0, 2.0]]);
        assert_eq!(diag_c.hu(0.5), [[-1.0, 0.0], [-2.0, 0.0]]);
        assert_eq!(diag_c.hu(0.75), [[0.0, -1.0], [0.0, -2.0]]);
        assert_eq!(diag_c.hu(1.0), [[1.0, 0.0], [2.0, 0.0]]);
        assert!(checku(&diag_c));

        // The left side is like locking the inner circle.
        let left_c = c.left();
        assert_eq!(left_c.hu(0.5), [[1.0, 0.0], [-2.0, 0.0]]);

        // The right side is like locking the outer circle.
        let top_c = c.top();
        assert_eq!(top_c.hu(0.5), [[-1.0, 0.0], [2.0, 0.0]]);
    }

    #[test]
    fn check_list() {
        let a = vec![Lerp(1.0, 2.0), Lerp(3.0, 4.0)];
        assert!(check(&a, 0));
        assert!(check(&a, 1));

        let b = vec![vec![Lerp(1.0, 2.0)]];
        assert!(check(&b, (0, 0)));
    }

    #[test]
    fn check_translate() {
        let a = Lerp(1.0, 2.0);
        let b = Translate(3.0);
        let c = Compose::new(a, b);
        assert!(checku2(&c));
        assert_eq!(c.hu([0.0, 0.0]), 1.0);
        assert_eq!(c.hu([1.0, 0.0]), 2.0);
        assert_eq!(c.hu([0.0, 1.0]), 4.0);
        assert_eq!(c.hu([1.0, 1.0]), 5.0);

        let a = Square::new(Lerp(1.0, 2.0), Lerp(3.0, 4.0));
        let b = Translate([10.0, 20.0]);
        let c = Compose::new(a.as_vec(), b);
        assert_eq!(c.hu([0.0, 0.0, 0.0]), [1.0, 3.0]);
        assert_eq!(c.hu([0.0, 0.0, 1.0]), [11.0, 23.0]);
        assert_eq!(c.hu([1.0, 0.0, 0.0]), [2.0, 3.0]);
        assert_eq!(c.hu([1.0, 0.0, 1.0]), [12.0, 23.0]);
        assert!(checku3(&c));

        let a = Cube::new(Lerp(0.0, 1.0), Lerp(0.0, 1.0), Lerp(0.0, 1.0));
        let b = Translate([10.0, 20.0, 30.0]);
        let c = Compose::new(a.as_vec(), b);
        assert_eq!(c.hu([0.0; 4]), [0.0; 3]);
        assert_eq!(c.hu([1.0; 4]), [11.0, 21.0, 31.0]);
        assert!(checku4(&c));

        let a = Cube4::new(Id, Id, Id, Id);
        let b = Translate([1.0, 2.0, 3.0, 4.0]);
        let c = Compose::new(a.as_vec(), b);
        assert_eq!(c.h([0.0; 4], [0.0; 5]), [0.0; 4]);
        assert_eq!(c.h([0.0; 4], [1.0; 5]), [1.0, 2.0, 3.0, 4.0]);

        let a = Lerp(1.0, 2.0);
        let b = Translate(10.0);
        let c = Compose::new(a, b);
        let d = Compose::new(c, b);
        assert_eq!(d.hu([0.0, 0.0, 0.0]), 1.0);
        assert_eq!(d.hu([0.0, 0.0, 1.0]), 11.0);
        assert_eq!(d.hu([0.0, 1.0, 0.0]), 11.0);
        assert_eq!(d.hu([0.0, 1.0, 1.0]), 21.0);
        assert_eq!(d.hu([1.0, 1.0, 1.0]), 22.0);
        assert!(checku3(&d));
    }
}
