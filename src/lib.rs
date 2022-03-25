#![allow(dead_code)]

use std::fmt;
use rand::{distributions::{Bernoulli, Distribution}, Rng};

#[derive(Debug, Clone, PartialEq)]
struct GameState([usize; Self::GRIDSIZE]);

impl GameState {
    const LENGTH: usize = 4;
    const WIDTH: usize = 4;
    const GRIDSIZE: usize = Self::LENGTH * Self::WIDTH;

    fn new() -> Self {
        Self([0; Self::GRIDSIZE])
    }

    fn slide(&mut self, direction: Direction) -> usize {
        match direction {
            Direction::Up => self.slide_up(),
            Direction::Down => self.slide_down(),
            Direction::Left => self.slide_left(),
            Direction::Right => self.slide_right(),
        }
    }

    fn slide_up(&mut self) -> usize {
        let mut score = 0;
        for column in 0..Self::LENGTH {
            for row in 0..Self::WIDTH {
                let index = row * Self::WIDTH + column;
                let mut value = self.0[index];
                let mut empty_index = None;
                for row_offset in 0..row {
                    let other_index = (row - row_offset - 1) * Self::WIDTH + column;
                    if self.0[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.0[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.0[empty_index] = value;
                    self.0[index] = 0;
                }
            }
        }
        score
    }

    fn slide_down(&mut self) -> usize {
        let mut score = 0;
        for column in 0..Self::LENGTH {
            for row in (0..Self::WIDTH).rev() {
                let index = row * Self::WIDTH + column;
                let mut value = self.0[index];
                let mut empty_index = None;
                for row_offset in 0..(Self::WIDTH - row - 1) {
                    let other_index = (row + row_offset + 1) * Self::WIDTH + column;
                    if self.0[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.0[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.0[empty_index] = value;
                    self.0[index] = 0;
                }
            }
        }
        score
    }

    fn slide_left(&mut self) -> usize {
        let mut score = 0;
        for row in 0..Self::WIDTH {
            for column in 0..Self::LENGTH {
                let index = row * Self::WIDTH + column;
                let mut value = self.0[index];
                let mut empty_index = None;
                for column_offset in 0..column {
                    let other_index = row * Self::WIDTH + column - column_offset - 1;
                    if self.0[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.0[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.0[empty_index] = value;
                    self.0[index] = 0;
                }
            }
        }
        score
    }

    fn slide_right(&mut self) -> usize {
        let mut score = 0;
        for row in 0..Self::WIDTH {
            for column in (0..Self::LENGTH).rev() {
                let index = row * Self::WIDTH + column;
                let mut value = self.0[index];
                let mut empty_index = None;
                for column_offset in 0..(Self::LENGTH - column - 1) {
                    let other_index = row * Self::WIDTH + column + column_offset + 1;
                    if self.0[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.0[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.0[empty_index] = value;
                    self.0[index] = 0;
                }
            }
        }
        score
    }

    fn add_random_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let mut empty_indices = Vec::new();
        for index in 0..Self::GRIDSIZE {
            if self.0[index] == 0 {
                empty_indices.push(index);
            }
        }
        if empty_indices.is_empty() {
            return;
        }
        let index = empty_indices[rng.gen_range(0..empty_indices.len())];
        let mut rng = rand::thread_rng();
        let bernoulli = Bernoulli::new(0.9).unwrap();
        self.0[index] = if bernoulli.sample(&mut rng) {
            2
        } else {
            4
        };
    }

    fn is_over(&self) -> bool {
        if self.0.iter().any(|&v| v == 0) {
            return false;
        }
        true
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..Self::WIDTH {
            for column in 0..Self::LENGTH {
                let index = row * Self::WIDTH + column;
                write!(f, "{:>4}", self.0[index])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    state: GameState,
    score: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut state = GameState::new();
        state.add_random_tile();
        state.add_random_tile();
        Self {
            state,
            score: 0,
        }
    }

    pub fn step(&mut self, direction: Direction) {
        self.score += self.state.slide(direction);
        self.state.add_random_tile();
    }

    pub fn start<F>(mut self, mut f: F) -> usize where F: FnMut(&[usize; GameState::GRIDSIZE]) -> Direction {
        while !self.state.is_over() {
            self.step(f(&self.state.0));
        }
        self.score
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score: {}\n{}", self.score, self.state)
    }
}
