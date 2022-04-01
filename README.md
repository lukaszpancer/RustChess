# RustChess
Python-chess inspired chess library in Rust

## Features
- abstract structures for representing basic chess entities, namely the board, pieces, and moves, 
- making moves and undoing moves, 
- keeping track of the previously made moves, 
- printing the current position in a terminal, in human-friendly form, 
- understanding all the chess rules, including lesser-known but a part of the official FIDE6 laws of chess, such as the en-passant, fifty-move rule or the threefold and fivefold repetition, 
- generating legal and pseudo-legal moves, 
- creating a move from the UCI7 as well as SAN8 format, 
- setting the board position using FEN9 format, 
- extracting the board position to the FEN format, 
- resetting the board to the starting position, 
- the internal representation of a chess game using a tree-like or graph structure, allowing for handling alternative sequences of moves deviating from the main line. A crucial feature when analyzing top-level games,  
- fast parsing of PGN files, 
- communication with UCI compatible chess engines, 
- querying online Syzygy tablebase API, 
- detection of absolute pins, 
- detection of checkmates, 
- detection of draws by insufficient material, 
- detection of checks and attacks, 
- counting number of moves without pushing the pawns and without captures for the fifty-move rule, 
- detecting threefold and fivefold repetitions. 

## Example

![image](https://user-images.githubusercontent.com/74537957/161228414-15425c35-7555-42e6-97d4-1f5a6ec58b57.png)

## UCI communication

![image](https://user-images.githubusercontent.com/74537957/161228539-ae3eea86-5e3f-4441-9597-723b03e7647b.png)

## Performance

### PGN parsing speed
![image](https://user-images.githubusercontent.com/74537957/161228701-6f4ed282-7761-450e-a038-916a7ec86abd.png)

Median times of parsing PGN files (single-core) compared to python-chess using PyPy and regular interpreter 

### Game Analysis speed
![image](https://user-images.githubusercontent.com/74537957/161229026-b8bf3033-d012-4c86-870f-28e207b6eef9.png)

Median time of benchmark code compared to python-chess using PyPy and regular interpreter 
