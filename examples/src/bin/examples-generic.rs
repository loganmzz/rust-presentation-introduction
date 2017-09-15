//Warning: Non-idiomatic Rust. Please PhantomData marker instead: https://doc.rust-lang.org/std/marker/struct.PhantomData.html

#[derive(Debug)]
struct Km;
#[derive(Debug)]
struct Mi;
#[derive(Debug)]
struct Distance<D>(u32, D);

#[derive(Clone, Debug)]
struct Hours;
#[derive(Debug)]
struct Speed<D,T=Hours>(Distance<D>, T);

trait Adaptable<T> {
    fn adapt(&self) -> T;
}

impl Adaptable<Distance<Mi>> for Distance<Km> {
    fn adapt(&self) -> Distance<Mi> {
        Distance(self.0 * 13 / 21, Mi)
    }
}

impl<DT,DS,S> Adaptable<Speed<DT,S>> for Speed<DS,S> where
    Distance<DS>: Adaptable<Distance<DT>>,
    S: Clone {

    fn adapt(&self) -> Speed<DT,S> {
        Speed(self.0.adapt(), self.1.clone())
    }

}

fn main() {
    let d100km = Distance(100, Km);
    println!("{:?} => {:?}", d100km, d100km.adapt());

    let d100kmh = Speed(d100km, Hours);
    println!("{:?} => {:?}", d100kmh, d100kmh.adapt());
}