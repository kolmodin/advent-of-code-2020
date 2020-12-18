use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub struct SearchIterator<State, Rep, MkRep, MkNextStates>
where
    State: Hash + Eq,
{
    rep: MkRep,
    next_states: MkNextStates,
    seen: HashSet<Rep>,
    queue: PriorityQueue<State, usize>,
}

impl<State, Rep, MkRep, MkNextStates> Iterator for SearchIterator<State, Rep, MkRep, MkNextStates>
where
    Rep: Hash + Eq,
    State: Hash + Eq + Debug,
    MkRep: Fn(&State) -> Rep,
    MkNextStates: Fn(&State) -> Vec<(usize, State)>,
{
    type Item = State;

    fn next(&mut self) -> Option<State> {
        while let Some((s, cost)) = self.queue.pop() {
            println!("-------\ninspecting {:#?}", s);

            let r = (self.rep)(&s);
            if self.seen.get(&r).is_some() {
                continue;
            }

            self.seen.insert(r);

            for (stepcost, next_state) in (self.next_states)(&s) {
                if self.seen.get(&(self.rep)(&next_state)).is_none() {
                    println!("new state! {:#?}", &next_state);
                    self.queue.push(next_state, cost + stepcost);
                }
            }

            return Some(s);
        }

        None
    }
}

pub fn search<State, Rep, MkRep, MkNextStates>(
    init: State,
    rep: MkRep,
    next_states: MkNextStates,
) -> SearchIterator<State, Rep, MkRep, MkNextStates>
where
    Rep: Hash + Eq,
    State: Hash + Eq,
    MkRep: Fn(&State) -> Rep,
    MkNextStates: Fn(&State) -> Vec<(usize, State)>,
{
    let seen = HashSet::<Rep>::new();
    let mut queue = PriorityQueue::new();
    queue.push(init, 0);

    SearchIterator {
        rep,
        next_states,
        seen,
        queue,
    }
}
#[cfg(test)]
mod dijkstra_tests {

    use std::collections::HashSet;
    use std::fs;
    use std::hash::Hash;

    use crate::dijkstra;

    #[derive(Hash, PartialEq, Eq, Debug)]
    struct S {
        index: i32,
        hops: i32,
    }

    fn parse_input(path: &str) -> Vec<i32> {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

        contents.lines().map(|ln| ln.parse().unwrap()).collect()
    }

    #[test]
    fn test_dijkstra() {
        let mut nums = parse_input("day10.txt");
        nums.push(0);

        let device = nums.iter().max().unwrap() + 3;
        nums.push(device);
        nums.sort_unstable();

        let hs: HashSet<i32> = nums.iter().cloned().collect();
        let init_s = S { index: 0, hops: 0 };
        let rep = |s: &S| s.index;

        let next_states = |s: &S| -> Vec<(usize, S)> {
            println!("next_states from {:#?}", s);
            (1..=3)
                .map(|i| S {
                    index: s.index + i,
                    hops: s.hops + 1,
                })
                .filter(|s| hs.get(&(s.index)).is_some())
                .map(|s| (1, s))
                .collect()
        };
        let is_done = |s: &S| s.index == device;

        assert_eq!(
            dijkstra::search(init_s, rep, next_states)
                .filter(is_done)
                .next(),
            Some(S {
                index: 155,
                hops: 101
            })
        );
    }
}