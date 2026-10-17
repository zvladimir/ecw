//! Current Shape: the RMS and average current and the duty cycle of a periodic current, for
//! the usual waveforms (half-sine, rectangular, trapezoidal, three kinds of triangle). Here are
//! the waveform kinds and the input fields each of them needs; `calc` does the calculation and
//! `times` fills in the missing one of the triangle times.

pub mod calc;
pub mod times;

use crate::msg;

use crate::domain::{Quantity, Text};

/// The shape of the current pulse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WaveformKind {
    /// A half-sine pulse.
    #[default]
    HalfSine,
    /// A rectangular pulse.
    Rectangular,
    /// A pulse whose current changes linearly from `Ip1` to `Ip2`.
    Trapezoidal,
    /// A triangle from zero to the peak and back.
    Triangular,
    /// A triangle between two levels above zero.
    TriangularOffset,
    /// Two triangles of opposite sign.
    BipolarTriangular,
}

impl WaveformKind {
    /// Every kind with its id in the request, in the order of the selector.
    pub const ALL: [(WaveformKind, &'static str); 6] = [
        (WaveformKind::HalfSine, "half_sine"),
        (WaveformKind::Rectangular, "rectangular"),
        (WaveformKind::Trapezoidal, "trapezoidal"),
        (WaveformKind::Triangular, "triangular"),
        (WaveformKind::TriangularOffset, "triangular_offset"),
        (WaveformKind::BipolarTriangular, "bipolar_triangular"),
    ];

    /// The kind with this id (`half_sine`, …); an unknown id reads as the default, the half-sine.
    pub fn parse(text: &str) -> Self {
        Self::ALL
            .iter()
            .find(|(_, id)| *id == text)
            .map(|(kind, _)| *kind)
            .unwrap_or_default()
    }

    /// The name of the kind as a message for the UI.
    pub fn name(self) -> Text {
        match self {
            WaveformKind::HalfSine => msg!("current_shape.kind.half_sine"),
            WaveformKind::Rectangular => msg!("current_shape.kind.rectangular"),
            WaveformKind::Trapezoidal => msg!("current_shape.kind.trapezoidal"),
            WaveformKind::Triangular => msg!("current_shape.kind.triangular"),
            WaveformKind::TriangularOffset => msg!("current_shape.kind.triangular_offset"),
            WaveformKind::BipolarTriangular => msg!("current_shape.kind.bipolar_triangular"),
        }
    }

    /// True for the three triangles: they are set by the times t1, t2, t3 and the period.
    pub fn is_triangular(self) -> bool {
        matches!(
            self,
            WaveformKind::Triangular
                | WaveformKind::TriangularOffset
                | WaveformKind::BipolarTriangular
        )
    }
}

/// An input field of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Var {
    /// The peak current; `Imax` for the triangles, `Ip1` for the trapezoid.
    Ip,
    /// The second level of the trapezoid.
    Ip2,
    /// The width of the pulse.
    Ton,
    /// The period T.
    Period,
    /// The lower level of a triangle that does not start at zero.
    Imin,
    /// The first time of a triangle, `t1`.
    T1,
    /// The second time of a triangle, `t2`.
    T2,
    /// The rest of the period, `t3`: `T = t1 + t2 + t3`.
    T3,
}

impl Var {
    /// The key of the field in the request.
    pub const fn key(self) -> &'static str {
        match self {
            Var::Ip => "ip",
            Var::Ip2 => "ip2",
            Var::Ton => "ton",
            Var::Period => "period",
            Var::Imin => "imin",
            Var::T1 => "t1",
            Var::T2 => "t2",
            Var::T3 => "t3",
        }
    }

    /// The caption of the field for this kind of waveform (`Ip [A]`); empty if the kind does not use the field.
    pub const fn label(self, kind: WaveformKind) -> &'static str {
        use WaveformKind::*;
        match (kind, self) {
            (HalfSine | Rectangular, Var::Ip) => "Ip [A]",
            (HalfSine | Rectangular, Var::Ton) => "ton [s]",
            (HalfSine | Rectangular, Var::Period) => "T [s]",

            (Trapezoidal, Var::Ip) => "Ip1 [A]",
            (Trapezoidal, Var::Ip2) => "Ip2 [A]",
            (Trapezoidal, Var::Ton) => "ton [s]",
            (Trapezoidal, Var::Period) => "T [s]",

            (Triangular, Var::Ip) => "Imax [A]",
            (Triangular, Var::T1) => "t1 [s]",
            (Triangular, Var::T2) => "t2 [s]",
            (Triangular, Var::T3) => "t3 [s]",
            (Triangular, Var::Period) => "T [s]",

            (TriangularOffset | BipolarTriangular, Var::Imin) => "Imin [A]",
            (TriangularOffset | BipolarTriangular, Var::Ip) => "Imax [A]",
            (TriangularOffset | BipolarTriangular, Var::T1) => "t1 [s]",
            (TriangularOffset | BipolarTriangular, Var::T2) => "t2 [s]",
            (TriangularOffset | BipolarTriangular, Var::T3) => "t3 [s]",
            (TriangularOffset | BipolarTriangular, Var::Period) => "T [s]",

            _ => "",
        }
    }

    /// The unit of the field.
    pub const fn unit(self) -> &'static str {
        match self {
            Var::Ip | Var::Ip2 | Var::Imin => "A",
            _ => "s",
        }
    }

    /// The physical quantity of the field.
    pub const fn quantity(self) -> Quantity {
        match self {
            Var::Ip | Var::Ip2 | Var::Imin => Quantity::Current,
            _ => Quantity::Time,
        }
    }

    /// Example input for the empty field.
    pub const fn hint(self) -> &'static str {
        match self {
            Var::Imin => "-1, -100m, -0.5",
            _ => self.quantity().input_hint(),
        }
    }
}

/// The fields a kind of waveform uses, in the order of the form.
pub const fn vars_for(kind: WaveformKind) -> &'static [Var] {
    use WaveformKind::*;
    match kind {
        HalfSine | Rectangular => &[Var::Ip, Var::Ton, Var::Period],
        Trapezoidal => &[Var::Ip, Var::Ip2, Var::Ton, Var::Period],
        Triangular => &[Var::Ip, Var::T1, Var::T2, Var::T3, Var::Period],
        TriangularOffset | BipolarTriangular => {
            &[Var::Imin, Var::Ip, Var::T1, Var::T2, Var::T3, Var::Period]
        }
    }
}
