# Lab2---Conway-s-Game-of-Life
Michelle Mejía 22596

# Conway's Game of Life

This is an implementation of Conway's Game of Life in Rust using the `minifb` crate for rendering.

## Description

Conway's Game of Life is a cellular automaton devised by mathematician John Horton Conway. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. The game consists of a grid of cells that can live, die, or multiply based on a few mathematical rules. The rules determine the state of the grid in the next generation based on the current state.

## Rules

1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed. If not, you can install them from [rustup.rs](https://rustup.rs/).

### Installing

Clone the repository:
https://github.com/michellemej22596/Lab2---GraficasGame.git

Run:
Cargo build
Cargo run

Enjoy!
![Game of Life](/Video.gif)

