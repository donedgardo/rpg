# 🎮 RPG Game Project 🌟

Welcome to the RPG Game Project! This repository contains the source code for a role-playing game (RPG) developed in Rust, leveraging powerful tools like Bevy, GGRS, and Matchbox for an immersive gaming experience.

## 🚀 Getting Started

To get started with this project, you'll need to have Rust installed on your machine. Follow these steps:

1. Clone the repository:

```shell
git clone https://github.com/donedgardo/rpg.git
```
2. Navigate to the project directory:
```shell
cd rpg
```
3. Build and run the project:
```shell
cargo run
```

## 🛠️ Tools and Libraries

### Bevy
Bevy is a refreshingly simple data-driven game engine built in Rust. It's used for creating cross-platform games with ease. Bevy's entity-component-system (ECS) architecture allows for highly concurrent and performant game logic.

### GGRS (Good Game Rollback System)
GGRS is a Rust implementation of GGPO-style rollback networking. It's designed for fast-paced, precise multiplayer experiences, ensuring smooth gameplay even over less-than-ideal network conditions.

### Matchbox
Matchbox is a networking library that complements GGRS. It provides the necessary tools for setting up and managing network connections, making it easier to implement multiplayer functionality in the game.

## 📂 Project Structure

- `src/`: Contains all the Rust source files.
- `main.rs`: The entry point of the game.
- `app_state.rs`, `arena.rs`, `camera.rs`, etc.: Various modules for game features.
- `assets/`: Contains assets like fonts and images used in the game.
- `start_db.ps1` and `start_signaling_server.ps1`: Scripts to set up the database and signaling server.

## 🎨 Assets

We use the Roboto font family for our game's typography. You can find these under the `assets/Roboto/` directory.

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.