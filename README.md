![Oria on computer](cover.jpeg)

# Starter : Oria 🐈❤️🦀
**Oria: Organized Rust API Architecture**

> ℹ️ (Oria is just the name of my cat, after I asked ChatGPT to find terms related to the project’s objectives.) 💁‍♂️

## Objective of project

It's a starter with specific architecture explained on [ARCHITECTURE.md](ARCHITECTURE.md)

## The objective is easy : 

- 🦀 Prove Rust is a very good choice to write new API on 2024
- 🕵️‍♂️ Prove module architecture is very interesting for development API with grow planned context
- 👷 Have starter ready to begin development on good condition

## Main choices

- The starter interface only **Rest API** because it's easier to implement on many interface and many program.
- **NoSQL** is better for scalability and is perfect for a lot of basic usage
- **Autonomous authenticate system** : Allow start project without deploy or subscribe authenticate system solution integration.

### Main dependencies choices

- [Shuttle](https://docs.shuttle.rs/getting-started/installation) For fast deployment
- [Axum](https://github.com/tokio-rs/axum) For modularity
- [Mongodb](https://crates.io/crates/mongodb) For scalability and easy to manage without migration

## Roadmap

- [ ] Add main structure with authenticate and user module
- [ ] Add rules configuration system
- [ ] Add logger dependency
- [ ] Add mailer module
- [ ] Add simple CRUD module example
- [ ] Scheduling article about this structure
- [ ] Add official front-end start integration on same principe

## Useful commands

| Command                    | Description              |
|----------------------------|--------------------------|
| cargo run test             | Run unit test            |
| cargo tarpaulin --out Html | Generate coverage report |

# Thank you

[Dorian Delorme](https://github.com/doriandel) for cover image edition