#![recursion_limit = "2000"]

use std::{marker::PhantomData, any::type_name};
trait Nat {
    fn zero() -> bool;
    type Dec: Nat;
    type Inc: Nat;
}

trait Zero {}

struct Z;
struct S<T: ?Sized + Nat>(PhantomData<T>);

impl Nat for Z {
    fn zero() -> bool {
        true
    }
    type Dec = Z;
    type Inc = S<Z>;
}

impl<T: Nat> Nat for S<T> {
    fn zero() -> bool {
        false
    }
    type Dec = T;
    type Inc = S<T>;
}

trait Count {
    fn count() -> u64;
}

impl Count for Z {
    fn count() -> u64 {
        0
    }
}
impl<T: Count + Nat> Count for S<T> {
    fn count() -> u64 {
        T::count() + 1
    }
}

fn eq<A: Nat, B: Nat>() -> bool {
    if A::zero() && B::zero() {
        true
    } else if (A::zero() && !B::zero()) || (!A::zero() && B::zero()) {
        false
    } else {
        eq::<A::Dec, B::Dec>()
    }
}

struct True;
struct False;
trait Bool {
    fn bool() -> bool;
}

impl Bool for True {
    fn bool() -> bool {
        true
    }
}
impl Bool for False {
    fn bool() -> bool {
        false
    }
}

trait PeanoEq<Rhs> {
    type Answer: Bool;
}

impl PeanoEq<Z> for Z {
    type Answer = True;
}

impl<T: Nat> PeanoEq<S<T>> for Z {
    type Answer = False;
}

impl<T: Nat> PeanoEq<Z> for S<T> {
    type Answer = False;
}

impl<T: Nat + PeanoEq<Y>, Y: Nat> PeanoEq<S<Y>> for S<T>
where
    S<T>: PeanoEq<Y>,
{
    type Answer = <T as PeanoEq<Y>>::Answer;
}

trait ReturnsNat {
    type Answer: Nat;
}

struct Add<Lhs, Rhs>(PhantomData<Lhs>, PhantomData<Rhs>);

impl<Lhs: Nat> ReturnsNat for Add<Lhs, Z> {
    type Answer = Lhs;
}

impl<Lhs: Nat, I: Nat> ReturnsNat for Add<Lhs, S<I>>
where
    Add<Lhs, I>: ReturnsNat,
{
    type Answer = S<<Add<Lhs, I> as ReturnsNat>::Answer>;
}

trait PeanoMul<Rhs> {
    type Answer: Nat;
}

impl<T> PeanoMul<T> for Z {
    type Answer = Z;
}

impl<X: Nat> PeanoMul<Z> for S<X> {
    type Answer = Z;
}

//impl<X: Nat, Y: Nat> PeanoMul<S<Y>> for S<X>
//where S<X>: PeanoMul<Y>  {
//    type Answer = <Add<Self, <Self as PeanoMul<Y>>::Answer> as ReturnsNat>::Answer;
//}

type N0 = Z;
type N1 = S<N0>;
type N2 = S<N1>;
type N3 = S<N2>;

trait List {
    type First;
}

struct Nil;
struct Cons<X, Xs: List>(X, Xs);

impl List for Nil {
    type First = Nil;
}
impl<X, Xs: List> List for Cons<X, Xs> {
    type First = X;
}

trait Mapping<R> {
    type Into;
}

trait MapList<R> {
    type Answer: List;
}

impl<R> MapList<R> for Nil {
    type Answer = Nil;
}

impl<X: Mapping<R>, Xs: List + MapList<R>, R> MapList<R> for Cons<X, Xs> {
    type Answer = Cons<<X as Mapping<R>>::Into, <Xs as MapList<R>>::Answer>;
}

trait Uncons {
    type Item;
    type List: List;
}
impl<X, Xs: List> Uncons for Cons<X, Xs> {
    type Item = X;
    type List = Xs;
}

trait ListSize {
    type Size: Nat;
}

impl ListSize for Nil {
    type Size = N0;
}

impl<X, Xs: List + ListSize> ListSize for Cons<X, Xs> {
    type Size = S<<Xs as ListSize>::Size>;
}

struct Zipper<L: List, R: List>(L, R);

trait ZipRight {
    type Result;
}

impl<L: List, X> ZipRight for Zipper<L, Cons<X, Nil>> {
    type Result = Zipper<Cons<X, L>, Cons<N0, Nil>>;
}

impl<L: List, X, Y, Xs: List> ZipRight for Zipper<L, Cons<X, Cons<Y, Xs>>> {
    type Result = Zipper<Cons<X, L>, Cons<Y, Xs>>;
}

trait ZipLeft {
    type Result;
}

impl<R: List, X, Xs: List> ZipLeft for Zipper<Cons<X, Xs>, R> {
    type Result = Zipper<Xs, Cons<X, R>>;
}

trait ZipSet<X> {
    type Result;
}

impl<L: List, X, Xs: List, U> ZipSet<X> for Zipper<L, Cons<U, Xs>> {
    type Result = Zipper<L, Cons<X, Xs>>;
}

trait ZipInc {
    type Result;
}

trait ZipDec {
    type Result;
}

/*trait U8 {
    type Inc;
    type Dec;
}

struct U8Z;
struct U8S<T: U8>(T);

type U8_1 = U8S<U8Z>;
type U8_2 = U8S<U8_1>;
type U8_3 = U8S<U8_2>;

impl U8 for U8Z {
    type Dec = U8255;
}*/

impl<L: List, Xs: List, N: Nat> ZipInc for Zipper<L, Cons<N, Xs>> {
    type Result = Zipper<L, Cons<S<N>, Xs>>;
}

impl<L: List, Xs: List, N: Nat> ZipDec for Zipper<L, Cons<S<N>, Xs>> {
    type Result = Zipper<L, Cons<N, Xs>>;
}

trait Debug {
    fn debug() -> String;
}

impl<X: Nat + Count, Xs: List, R: List> Debug for Zipper<Cons<X, Xs>, R>
where
    Zipper<Xs, R>: Debug,
{
    fn debug() -> String {
        format!(
            "{} {}",
            <X as Count>::count(),
            <Zipper::<Xs, R> as Debug>::debug()
        )
    }
}

impl<X: Nat + Count, Xs: List> Debug for Zipper<Nil, Cons<X, Xs>>
where
    Zipper<Nil, Xs>: Debug,
{
    fn debug() -> String {
        format!("{} {}", <X as Count>::count(), Zipper::<Nil, Xs>::debug())
    }
}

impl Debug for Zipper<Nil, Nil> {
    fn debug() -> String {
        String::new()
    }
}

trait SearchForEnd {
    type Result: List;
}

impl<Xs: List> SearchForEnd for Cons<LoopEnd, Xs> {
    type Result = Xs;
}
impl<Xs: List + SearchForEnd> SearchForEnd for Cons<LoopStart, Xs>
where
    <Xs as SearchForEnd>::Result: SearchForEnd,
{
    type Result = <<Xs as SearchForEnd>::Result as SearchForEnd>::Result;
}
impl<Xs: List + SearchForEnd> SearchForEnd for Cons<MoveLeft, Xs> {
    type Result = <Xs as SearchForEnd>::Result;
}

impl<Xs: List + SearchForEnd> SearchForEnd for Cons<MoveRight, Xs> {
    type Result = <Xs as SearchForEnd>::Result;
}
impl<Xs: List + SearchForEnd> SearchForEnd for Cons<Increment, Xs> {
    type Result = <Xs as SearchForEnd>::Result;
}
impl<Xs: List + SearchForEnd> SearchForEnd for Cons<Decrement, Xs> {
    type Result = <Xs as SearchForEnd>::Result;
}
impl<Xs: List + SearchForEnd> SearchForEnd for Cons<Output, Xs> {
    type Result = <Xs as SearchForEnd>::Result;
}

struct MoveLeft;
struct MoveRight;
struct Increment;
struct Decrement;
struct LoopStart;
struct LoopEnd;
struct Output;


trait Instr<Tape, Stack, Program> {
    type Tape;
    type Stack: List;
    type Program: List;
    type Output;
}

impl<Tape: ZipLeft, S: List, Xs: List> Instr<Tape, S, Cons<Self, Xs>> for MoveLeft {
    type Tape = Tape::Result;
    type Stack = S;
    type Program = Xs;
    type Output = Nil;
}

impl<Tape: ZipRight, S: List, Xs: List> Instr<Tape, S, Cons<Self, Xs>> for MoveRight {
    type Tape = Tape::Result;
    type Stack = S;
    type Program = Xs;
    type Output = Nil;
}

impl<Tape: ZipInc, S: List, Xs: List> Instr<Tape, S, Cons<Self, Xs>> for Increment {
    type Tape = Tape::Result;
    type Stack = S;
    type Program = Xs;
    type Output = Nil;
}

impl<Tape: ZipDec, S: List, Xs: List> Instr<Tape, S, Cons<Self, Xs>> for Decrement {
    type Tape = Tape::Result;
    type Stack = S;
    type Program = Xs;
    type Output = Nil;
}

trait ZipIsNonZero {}
impl<L: List, R: List, T: Nat> ZipIsNonZero for Zipper<L, Cons<S<T>, R>> {}

impl<L: List, R: List, S: List, Xs: List + SearchForEnd> Instr<Zipper<L, Cons<N0, R>>, S, Cons<Self, Xs>>
    for LoopStart
{
    type Tape = Zipper<L, Cons<N0, R>>;
    type Stack = S;
    type Program = <Xs as SearchForEnd>::Result;
    type Output = Nil;
}

impl<Tape: ZipIsNonZero, S: List, Xs: List + SearchForEnd> Instr<Tape, S, Cons<Self, Xs>>
    for LoopStart
{
    type Tape = Tape;
    type Stack = Cons<Cons<Self, Xs>, S>;
    type Program = Xs;
    type Output = Nil;
}

impl<L: List, R: List, X, Sx: List, Xs: List>
    Instr<Zipper<L, Cons<N0, R>>, Cons<X, Sx>, Cons<Self, Xs>> for LoopEnd
{
    type Tape = Zipper<L, Cons<N0, R>>;
    type Stack = Sx;
    type Program = Xs;
    type Output = Nil;
}

impl<Tape: ZipIsNonZero, X: List, Sx: List, Xs: List>
    Instr<Tape, Cons<X, Sx>, Cons<Self, Xs>> for LoopEnd
{
    type Tape = Tape;
    type Stack = Sx;
    type Program = X;
    type Output = Nil;
}

impl<L: List, R: List, X, S: List, Xs: List>
    Instr<Zipper<L, Cons<X, R>>, S, Cons<Self, Xs>> for Output
{
    type Tape = Zipper<L, Cons<X, R>>;
    type Stack = S;
    type Program = Xs;
    type Output = X;
}

#[cfg(test)]
mod test {
    use crate::{Cons, Increment, LoopEnd, Nil, SearchForEnd};
    #[test]
    fn search() {
        assert_eq!(
            std::any::TypeId::of::<
                <Cons<Increment, Cons<LoopEnd, Cons<Increment, Nil>>> as SearchForEnd>::Result,
            >(),
            std::any::TypeId::of::<Cons<Increment, Nil>>()
        );
    }
}

struct Interpreter<Tape, Stack: List, Program: List, Output>(Tape, Stack, Program, Output);
struct Terminated;
struct Running;

trait Step {
    type Next;
    type Status;
}

trait Append<T> {
    type Result: List;
}

impl<T: List> Append<T> for Nil {
    type Result = T;
}

impl<T: List> Append<T> for Z {
    type Result = Cons<Z, T>;
}

impl<T: List, N: Nat> Append<T> for S<N> {
    type Result = Cons<Self, T>;
}

//impl<X, Xs: List, L> Append<L> for Cons<X, Xs> {
//    type Result = Cons<L, Cons<X, Xs>>;
//}

impl<T, S: List, O> Step for Interpreter<T, S, Nil, O> {
    type Next = Self;
    type Status = Terminated;
}

impl<T, S: List, In: Instr<T, S, Cons<In, Xs>>, Xs: List, O: List> Step for Interpreter<T, S, Cons<In, Xs>, O> 
where In::Output: Append<O> {
    type Next = Interpreter<In::Tape, In::Stack, In::Program, <In::Output as Append<O>>::Result>;
    type Status = Running;
}

trait FullyExecute {
    type Final;
}

impl<T, S: List, O> FullyExecute for Interpreter<T, S, Nil, O> {
    type Final = Self;
}

impl<T, S: List, In: Instr<T, S, Cons<In, Xs>>, Xs: List, O> FullyExecute for Interpreter<T, S, Cons<In, Xs>, O>
where Interpreter<<In as Instr<T, S, Cons<In, Xs>>>::Tape, <In as Instr<T, S, Cons<In, Xs>>>::Stack, <In as Instr<T, S, Cons<In, Xs>>>::Program, <In::Output as Append<O>>::Result>: FullyExecute,
      In::Output: Append<O>
{
    type Final = <Interpreter<In::Tape, In::Stack, In::Program, <In::Output as Append<O>>::Result> as FullyExecute>::Final;
}
trait GetOutput {
    type Output;
}

impl<T, S: List, P: List, O> GetOutput for Interpreter<T, S, P, O> {
    type Output = O;
}

type Program = Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<LoopStart,Cons<MoveRight,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<LoopStart,Cons<MoveRight,Cons<Increment,Cons<Increment,Cons<MoveRight,Cons<Increment,Cons<Increment,Cons<Increment,Cons<MoveRight,Cons<Increment,Cons<Increment,Cons<Increment,Cons<MoveRight,Cons<Increment,Cons<MoveLeft,Cons<MoveLeft,Cons<MoveLeft,Cons<MoveLeft,Cons<Decrement,Cons<LoopEnd,Cons<MoveRight,Cons<Increment,Cons<MoveRight,Cons<Increment,Cons<MoveRight,Cons<Decrement,Cons<MoveRight,Cons<MoveRight,Cons<Increment,Cons<LoopStart,Cons<MoveLeft,Cons<LoopEnd,Cons<MoveLeft,Cons<Decrement,Cons<LoopEnd,Cons<MoveRight,Cons<MoveRight,Cons<Output,Cons<MoveRight,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Output,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Output,Cons<Output,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Output,Cons<MoveRight,Cons<MoveRight,Cons<Output,Cons<MoveLeft,Cons<Decrement,Cons<Output,Cons<MoveLeft,Cons<Output,Cons<Increment,Cons<Increment,Cons<Increment,Cons<Output,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Output,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Decrement,Cons<Output,Cons<MoveRight,Cons<MoveRight,Cons<Increment,Cons<Output,Cons<MoveRight,Cons<Increment,Cons<Increment,Cons<Output,Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;
//type Program = Cons<Increment, Cons<Increment, Cons<Output, Cons<LoopStart, Cons<Decrement, Cons<LoopEnd, Nil>>>>>>;

type DefaultInterpreter<Program> = Interpreter<Zipper<Nil, Cons<N0, Nil>>, Nil, Program, Nil>;
type ProgramResult = <DefaultInterpreter<Program> as FullyExecute>::Final;

type StepOne = <DefaultInterpreter<Program> as Step>::Next;
type Step2 = <StepOne as Step>::Next;
type Step3 = <Step2 as Step>::Next;
type Step4 = <Step3 as Step>::Next;
type Step5 = <Step4 as Step>::Next;

trait ToInts {
    fn into(v: Vec<u64>) -> Vec<u64>;
    fn to_string() -> String {
        let mut output = Self::into(Vec::new());
        output.reverse();
        let bytes: Vec<u8> = output.into_iter().flat_map(u8::try_from).collect();
        String::from_utf8(bytes).unwrap()
    }
}

impl ToInts for Nil {
    fn into(v: Vec<u64>) -> Vec<u64> {v}
}

impl<N: Nat + Count, Xs: List + ToInts> ToInts for Cons<N, Xs> {
    fn into(mut v: Vec<u64>) -> Vec<u64> {v.push(N::count()); <Xs as ToInts>::into(v) }
}

fn main() {
    println!("ok");
    println!("{}", <Z as PeanoEq<Z>>::Answer::bool());
    println!("{}", <N1 as PeanoEq<Z>>::Answer::bool());
    println!("{}", <Z as PeanoEq<N1>>::Answer::bool());
    println!("{}", <N1 as PeanoEq<N1>>::Answer::bool());
    println!();
    println!("{}", eq::<Z, S<Z>>());
    println!("{}", eq::<S<Z>, Z>());
    println!("{}", eq::<N1, N1>());
    println!("{}", N3::count());
    println!("{}", <Add::<N3, N2> as ReturnsNat>::Answer::count());
    println!(
        "1 + 1 ==  ? {}",
        <Add::<N1, N1> as ReturnsNat>::Answer::count()
    );
    println!(
        "1 + 1 == 2? {}",
        <<Add::<N1, N1> as ReturnsNat>::Answer as PeanoEq<N2>>::Answer::bool()
    );
    println!("[] {}", <Nil as ListSize>::Size::count());
    println!("[0] {}", <Cons<N0, Nil> as ListSize>::Size::count());
    println!(
        "[0 0] {}",
        <Cons<N0, Cons<N0, Nil>> as ListSize>::Size::count()
    );

    println!("{:?}", std::any::TypeId::of::<S<S<S<Z>>>>());
    type Test = Zipper<Nil, Cons<N0, Nil>>;
    println!(
        "{:?}",
        <<<Test as ZipRight>::Result as ZipInc>::Result as Debug>::debug()
    );
    println!("{}", type_name::<StepOne>());
    println!("{}", type_name::<Step5>());
    println!("{}", type_name::<ProgramResult>());
    println!("{}", <<ProgramResult as GetOutput>::Output as ToInts>::to_string())
}
