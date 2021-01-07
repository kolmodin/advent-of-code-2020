
use std::collections::HashSet;
use std::hash::Hash;

pub struct DfsIterator<State, MkNext, MkRep, Rep> {
    rep: MkRep,
    seen: HashSet<Rep>,
    queue: Vec<State>,
    next: MkNext,
}

impl<State, MkNext, MkRep, Rep> Iterator for DfsIterator<State, MkNext, MkRep, Rep>
where
    MkNext: Fn(&State) -> Vec<State>,
    MkRep: Fn(&State) -> Rep,
    Rep: Eq + Hash,
{
    type Item = State;
    fn next(&mut self) -> Option<State> {
        match self.queue.pop() {
            None => None,
            Some(state) => {
                let rep = (self.rep)(&state);
                if self.seen.get(&rep).is_some() {
                    return self.next();
                }
                let next_states = (self.next)(&state);
                self.queue.extend(next_states.into_iter().rev());
                self.seen.insert(rep);
                Some(state)
            }
        }
    }
}

pub fn dfs<State, MkNext, MkRep, Rep>(
    init: State,
    rep: MkRep,
    next: MkNext,
) -> DfsIterator<State, MkNext, MkRep, Rep>
where
    MkNext: Fn(&State) -> Vec<State>,
    MkRep: Fn(&State) -> Rep,
    Rep: Hash + Eq,
{
    DfsIterator {
        seen: HashSet::new(),
        queue: vec![init],
        next,
        rep,
    }
}

// Second attempt with traits.

// A trait which can get the next steps from a state,
// and convert a state into some representation for already seen states.
pub trait SearchHelper {
    type State;
    type Rep: Eq + Hash;

    fn next(&self, state: &Self::State) -> Vec<Self::State>;
    fn rep(&self, state: &Self::State) -> Self::Rep;
}

// The struct which can produce the iterator.
pub struct DFSIterator<T>
where
    T: SearchHelper,
{
    helper: T,
    seen: HashSet<T::Rep>,
    queue: Vec<T::State>,
}

impl<T> Iterator for DFSIterator<T>
where
    T: SearchHelper,
{
    type Item = T::State;
    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop() {
            None => None,
            Some(state) => {
                let rep = self.helper.rep(&state);
                if self.seen.get(&rep).is_some() {
                    return self.next();
                }
                let next_states = self.helper.next(&state);
                self.queue.extend(next_states.into_iter().rev());
                self.seen.insert(rep);
                Some(state)
            }
        }
    }
}

pub fn dfs_search<T>(helper: T, init: T::State) -> DFSIterator<T>
where
    T: SearchHelper,
{
    DFSIterator {
        helper,
        queue: vec![init],
        seen: HashSet::new(),
    }
}

/*
use search::SearchHelper;

struct SomeExample {}

impl SearchHelper for SomeExample {
    type State = i32;
    type Rep = i32;

    fn next(&self, state: &i32) -> Vec<i32> {
        vec![*state, state + 1, state * 2]
    }

    fn rep(&self, state: &i32) -> i32 {
        *state
    }
}

fn main() {
    let a = search::dfs(0, |i| *i, |i| vec![i * 5, i + 2]);
    for i in a.take(10) {
        println!("{}", i);
    }

    for i in search::dfs_search(SomeExample {}, 0).take(10) {
        println!("{}", i);
    }

    let ex = SomeExample {};
    let b = search::dfs(0, |i| ex.rep(i), |i| ex.next(i));
    for i in b.take(10) {
        println!("{}", i);
    }
}
*/
