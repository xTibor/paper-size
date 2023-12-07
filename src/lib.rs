#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    // ISO 216
    IsoA(isize),
    IsoB(isize),
    IsoC(isize),

    // JIS P 0138
    JisA(isize),
    JisB(isize),

    // SIS 014711
    SisA(isize),
    SisB(isize),
    SisC(isize),
    SisD(isize),
    SisE(isize),
    SisF(isize),
    SisG(isize),
    SisH(isize),

    #[deprecated(note = "use metric formats instead")]
    Legal,
    #[deprecated(note = "use metric formats instead")]
    Letter,
    #[deprecated(note = "use metric formats instead")]
    Tabloid,

    #[deprecated(note = "use metric formats instead")]
    AnsiA,
    #[deprecated(note = "use metric formats instead")]
    AnsiB,
    #[deprecated(note = "use metric formats instead")]
    AnsiC,
    #[deprecated(note = "use metric formats instead")]
    AnsiD,
    #[deprecated(note = "use metric formats instead")]
    AnsiE,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

trait Root {
    fn root(self, degree: f64) -> f64;
}

impl Root for f64 {
    fn root(self, degree: f64) -> f64 {
        self.powf(1.0 / degree)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl Format {
    pub fn size(&self, orientation: Orientation) -> (f64, f64) {
        match orientation {
            Orientation::Portrait => self.portrait_size(),
            Orientation::Landscape => self.landscape_size(),
        }
    }

    #[rustfmt::skip]
    #[allow(deprecated)]
    pub fn portrait_size(&self) -> (f64, f64) {
        let metric_size = |n: isize, a: f64, m: f64| {
            let r = 1.0 / 2.0.root(2.0);
            (
                a * m * r.powf(n as f64 + 1.0),
                a * m * r.powf(n as f64 + 0.0),
            )
        };

        let inch_to_mm = |n: f64| n * 25.4;

        match self {
            Format::IsoA(n) => metric_size(*n, 2.0.root(4.0), 1000.0),
            Format::IsoB(n) => metric_size(*n, 2.0.root(2.0), 1000.0),
            Format::IsoC(n) => metric_size(*n, 8.0.root(8.0), 1000.0),

            Format::JisA(n) => metric_size(*n, 2.0.root(4.0), (1000.0 * 1000.0).root(2.0)),
            Format::JisB(n) => metric_size(*n, 2.0.root(4.0), (1000.0 * 1500.0).root(2.0)),

            Format::SisA(n) => metric_size(*n, 2.0.root(16.0).powf( 4.0), 1000.0),
            Format::SisE(n) => metric_size(*n, 2.0.root(16.0).powf( 5.0), 1000.0),
            Format::SisC(n) => metric_size(*n, 2.0.root(16.0).powf( 6.0), 1000.0),
            Format::SisG(n) => metric_size(*n, 2.0.root(16.0).powf( 7.0), 1000.0),
            Format::SisB(n) => metric_size(*n, 2.0.root(16.0).powf( 8.0), 1000.0),
            Format::SisF(n) => metric_size(*n, 2.0.root(16.0).powf( 9.0), 1000.0),
            Format::SisD(n) => metric_size(*n, 2.0.root(16.0).powf(10.0), 1000.0),
            Format::SisH(n) => metric_size(*n, 2.0.root(16.0).powf(11.0), 1000.0),

            Format::Legal   => (inch_to_mm( 8.5), inch_to_mm(14.0)),
            Format::Letter  => (inch_to_mm( 8.5), inch_to_mm(11.0)),
            Format::Tabloid => (inch_to_mm(11.0), inch_to_mm(17.0)),

            Format::AnsiA => (inch_to_mm( 8.5), inch_to_mm(11.0)),
            Format::AnsiB => (inch_to_mm(11.0), inch_to_mm(17.0)),
            Format::AnsiC => (inch_to_mm(17.0), inch_to_mm(22.0)),
            Format::AnsiD => (inch_to_mm(22.0), inch_to_mm(34.0)),
            Format::AnsiE => (inch_to_mm(34.0), inch_to_mm(44.0)),
        }
    }

    pub fn landscape_size(&self) -> (f64, f64) {
        let (width, height) = self.portrait_size();
        (height, width)
    }
}
