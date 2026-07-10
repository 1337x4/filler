# Filler

A Rust-based AI robot for the **Filler** algorithmic game. Two robots compete on a 2D grid (the Anfield), taking turns placing randomly-shaped pieces with exactly one cell of overlap onto their existing territory. The player occupying the most cells at the end wins.

## Project Structure

```
filler/
├── Dockerfile
├── solution/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs       # Entry point & game loop
│       ├── parser.rs     # Stdin parsing (Anfield + pieces)
│       ├── board.rs      # Board state & territory tracking
│       ├── strategy.rs   # Piece placement strategy
│       └── tests.rs      # Unit tests
```

## Building & Running (Docker)

1. Download and extract the provided `filler.zip` (contains `game_engine`, maps, robots, Dockerfile).
2. Place this repo's `solution/` folder inside the extracted `docker_image/` folder.
3. Build the Docker image:
   ```bash
   docker build -t filler .
   ```
4. Run the container (mounts your solution directory):
   ```bash
   docker run -v "$(pwd)/solution":/filler/solution -it filler
   ```
5. Inside the container, compile the robot:
   ```bash
   cd /filler/solution && cargo build --release
   ```
6. Run the game:
   ```bash
   cd /filler
   ./game_engine -f maps/map01 -p1 robots/bender -p2 solution/target/release/filler_robot
   ```

## Running Tests

```bash
cd solution
cargo test
```

## Strategy

The robot uses a **greedy expansion** strategy:
- Scans every valid placement position (exactly one overlapping cell with own territory, zero overlaps with opponent).
- Scores each candidate position by counting how many new cells it would add closest to the center of the Anfield (to grow toward the opponent and maximise area).
- Picks the highest-scoring valid placement.
- Falls back to `0 0` if no valid placement exists.
