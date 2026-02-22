This project has been made by Pierre Ferrari in the context of a technical assignment for the position of Online Programmer at Carpool Studio.

The project contains a REST API in Rust made using the Axum framework. It also uses a Redis instance (using Docker) to manage the data in an efficent way.

To build the project, use the "cargo build" command in the terminal.

To start the project, first start Redis with the command "docker compose up -d" (Docker must be installed and running). You can then start the server using the "cargo run" command.

This API can be tested with this demo project on Unreal 5.6 : https://github.com/PapyPierre/Carpool_tech_test_ue_proj.git
