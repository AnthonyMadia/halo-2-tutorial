use std::marker::PhantomData;
use halo2_proofs::circuit::Value;
use halo2_proofs:: {
    arithmetic::FieldExt,
    circuit::{Cell, Chip, Layouter, SimpleFloorPlanner},
    plonk::{Advice, Assigned, Circuit, Column, ConstraintSystem, Error, Fixed, Instance},
    poly::Rotation,
};

// Steps:
// 1) Define config 
// 2) Wrap config with some marker in a struct (and build a new() for it)
// 3) Define a trait that will contain the functionality you want in your chip
// 4) implement those composer functions on your chip

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Clone)]
// implement config as a struct
struct TutorialConfig {
    // now columsn and types
    l: Column<Advice>,
    r: Column<Advice>,
    o: Column<Advice>,

    sl: Column<Fixed>,
    sr: Column<Fixed>,
    sm: Column<Fixed>,
    so: Column<Fixed>,
    PI: Column<Instance>,
}

struct TutorialChip<F: FieldExt> {
    config: TutorialConfig,
    marker: PhantomData<F>,
}

// impl keyword - implement some functionality for a type 
    // defines implementations on types 
impl<F: FieldExt> TutorialChip<F> {
    fn new(config: TutorialConfig) -> Self {
        TutorialChip {
            config,
            marker: PhantomData,
        }
    }
}

// a trait defines functionality a particular type has and can share with other types 
// more about traits: https://doc.rust-lang.org/book/ch10-02-traits.html 
//
// a `Cell` refers to a single location in the table of values we are building, such as
// one of the entries of `r`  or one of the entries of `s_o`
trait TutorialComposer<F: FieldExt> {
    fn raw_add<FM>(
        &self,
        layouter: &mut impl Layouter<F>,
        f: FM,
    ) -> Result<(Cell, Cell, Cell), Error>
    where 
        FM: FnMut() -> Value<(Assigned<F>, Assigned<F>, Assigned<F>)>;

    fn raw_multiply<FM>(
        &self,
        layouter: &mut impl Layouter<F>,
        f: FM,
    ) -> Result<(Cell, Cell, Cell), Error>
    where
        FM: FnMut() -> Value(Assigned<F>, Assigned<F>, Assigned<F>)

    // ensure two wire values are the same, in effect connecting the wire to each other
    fn copy(&self, layouter: &mut impl Layouter<F>, a: Cell, b: Cell) -> Result<(), Error>;

    // exposes a number as a public input to the circuit
    fn expose_public(&self, layouter: &mut Layouter<F>, cell: Cell, row: usize,) -> Result<(), Error>;

}
