#![feature(globs)]
extern crate num;

use rover::*;


mod rover;

#[test]
fn test_rover_move() {
	let mut rover1 = Rover::new(1, 2, N);
	rover1.travel("LMLMLMLMM");
	rover1.print();

    assert!(rover1.position.x == 1);
}

fn main() {
	println!("Starting rover navaigation...");

	println!("  Grid == 5x5");
	println!("  Robot 1 starts @ \\{1,2,N\\}");
	println!("  Robot 1 moves: LMLMLMLMM");
	println!("  Robot 2 starts @ \\{3,3,E\\}");
	println!("  Robot 2 moves: MMRMMRMRRM");

	let mut rover1 = Rover::new(1, 2, N);
	rover1.travel("LMLMLMLMM");
	rover1.print();

	let mut rover2 = Rover::new(3, 3, E);
	rover2.travel("MMRMMRMRRM");
	rover2.print();

	let one_half = ::num::rational::Ratio::new(1, 2);
	println!("1/2 = {}", one_half);

}