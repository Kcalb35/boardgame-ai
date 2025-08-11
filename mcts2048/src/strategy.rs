use crate::game_logic::Direction;
use crate::game_logic::GameState;
use rand::{Rng, rng};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub trait GameStrategy {
    fn next_move(&self, state: &GameState) -> Direction;
}

pub struct RandomStrategy;

impl GameStrategy for RandomStrategy {
    fn next_move(&self, _state: &GameState) -> Direction {
        let mut rng = rng();
        match rng.random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

pub struct Node {
    max_score: f64,
    visit: f64,
    total_value: f64,
    from_action: Direction,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    untried_actions: Vec<Direction>,
}

impl Node {
    pub fn new(action: Direction, parent: Option<Weak<RefCell<Node>>>) -> Self {
        Node {
            max_score: 0.0,
            visit: 0.0,
            total_value: 0.0,
            from_action: action,
            parent,
            children: vec![],
            untried_actions: vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        }
    }

    pub fn update(&mut self, value: f64) {
        self.visit += 1.0;
        self.total_value += value;
        if value > self.max_score {
            self.max_score = value;
        }
        if let Some(ptr) = &self.parent {
            ptr.upgrade().unwrap().borrow_mut().update(value);
        }
    }

    pub fn is_fully_expanded(&self) -> bool {
        self.untried_actions.is_empty()
    }

    pub fn best_child(&self, c_param: f64) -> Rc<RefCell<Node>> {
        let scores = self
            .children
            .iter()
            .map(|c| {
                let c = c.borrow();
                c.total_value / c.visit
                    + c_param * self.max_score * (2.0 * self.visit.ln() / c.visit).sqrt()
            })
            .collect::<Vec<_>>();

        let (idx, _) = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        self.children[idx].clone()
    }
}

pub struct SimpleRandomStrategy {
    pub depth: u64,
}

impl GameStrategy for SimpleRandomStrategy {
    fn next_move(&self, state: &GameState) -> Direction {
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut scores = [0.0; 4];

        for i in 0..directions.len() {
            for _ in 0..self.depth {
                let mut state = state.clone();
                state.move_tiles(directions[i]);

                while !state.is_game_over() {
                    let direction = RandomStrategy.next_move(&state);
                    state.move_tiles(direction);
                }

                scores[i] += state.score as f64;
            }
        }

        let (idx, _) = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        directions[idx]
    }
}

pub struct MctsStrategy {
    pub tries: u64,
    pub c_param: f64,
}

pub fn expand(node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let action = node.borrow_mut().untried_actions.pop().unwrap();
    let child = Rc::new(RefCell::new(Node::new(action, Some(Rc::downgrade(&node)))));
    node.borrow_mut().children.push(child.clone());
    child
}

pub fn simulate(state: &mut GameState) {
    while !state.is_game_over() {
        let direction = RandomStrategy.next_move(state);
        state.move_tiles(direction);
    }
}

impl GameStrategy for MctsStrategy {
    fn next_move(&self, state: &GameState) -> Direction {
        let root = Rc::new(RefCell::new(Node::new(Direction::Down, None)));

        for _ in 0..self.tries {
            let mut state = state.clone();
            let mut ptr = root.clone();

            while ptr.borrow().is_fully_expanded() {
                let child = ptr.borrow().best_child(self.c_param);
                state.move_tiles(child.borrow().from_action);
                ptr = child;
            }

            ptr = expand(ptr);
            state.move_tiles(ptr.borrow().from_action);
            simulate(&mut state);
            ptr.borrow_mut().update(state.score as f64);
        }

        for c in root.borrow().children.iter() {
            println!(
                "{:?}\t{}\t{:.1}",
                c.borrow().from_action,
                c.borrow().visit,
                c.borrow().total_value as f64 / c.borrow().visit
            );
        }

        let child = root.borrow().best_child(0.0);
        child.borrow().from_action
    }
}
