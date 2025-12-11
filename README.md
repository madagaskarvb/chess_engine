# How to Play Against the Rust Chess Engine

This engine supports the **UCI (Universal Chess Interface)** protocol, making it compatible with most popular chess GUIs (like Arena, Cute Chess, Fritz, etc.).

## 1. Build the Engine
First, compile the engine to create an executable file. Open your terminal in the `chess` directory and run:

```bash
cargo build --release
```

This will create an optimized executable file located at:
`chess/target/release/chess.exe` (Windows) or `chess/target/release/chess` (Linux/macOS).

## 2. Install a Chess GUI
If you don't have one, download a free UCI-compatible Chess GUI, such as:
- **Arena Chess GUI** (Windows/Linux)
- **Cute Chess** (Cross-platform)
- **Banksia GUI**

## 3. Connect the Engine to the GUI
1.  Open your Chess GUI.
2.  Find the option to **"Install New Engine"** or **"Manage Engines"**.
3.  Browse and select the `chess.exe` file you built in Step 1.
4.  When asked for the protocol, select **UCI**.
5.  Click "OK" or "Apply".

## 4. Start a Game
1.  Go to the "New Game" menu in your GUI.
2.  Select your Rust engine as one of the players (or as the opponent).
3.  Set the time control and start playing!

