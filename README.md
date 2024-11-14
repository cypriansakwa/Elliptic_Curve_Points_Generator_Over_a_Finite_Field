# Elliptic_Curve_Points_Generator_Over_a_Finite_Field

This Rust program computes and displays all the points on an elliptic curve defined over a finite field using a brute-force approach. The elliptic curve equation used is:

$y^2 \equiv x^3 + ax + b \pmod{p}$

The program iterates through all possible integer values of $x$ and $y$ within the finite field $\mathbb{F}_p$ and finds pairs $(x, y)$ that satisfy the elliptic curve equation.

## Table of Contents
- [How It Works](#how-it-works)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Example Output](#example-output)
- [Contributing](#contributing)
- [License](#license)

## How It Works

1. **Elliptic Curve Equation**: The program solves the elliptic curve equation:

   $y^2 \equiv x^3 + ax + b \pmod{p}$

3. **Iterating Over Possible Points**: The program iterates over all integers $x$ and $y$ in the range $[0, p-1]$. For each value of $x$, it computes the right-hand side of the equation and checks if there exists a corresponding $y$ such that $y^2 \equiv x^3 + ax + b \pmod{p}$.

4. **Finding Points**: If a valid $y$ is found, the program also considers the point $(x, p - y)$ if $y \neq 0$.

## Requirements

- [Rust](https://www.rust-lang.org/) (latest stable version)

## Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/yourusername/elliptic_curve_points_finder.git
    cd elliptic_curve_points_finder
    ```

2. **Build the project**:
    ```bash
    cargo build
    ```

3. **Run the program**:
    ```bash
    cargo run
    ```

## Usage

To execute the program, simply run:

```bash
cargo run
