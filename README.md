# File Server with Rocket

This project is a simple file server built with Rust and Rocket framework. It provides REST API endpoints to upload, download, list, and delete files.

## Prerequisites

- Rust and Cargo installed
- Clone this repository

## Getting Started

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/file_server.git
   cd file_server
   ```

2. Update `Cargo.toml`:

   ```toml
   [dependencies]
   rocket = { version = "0.5.1", features = ["json"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   tokio = { version = "1", features = ["full"] }
   ```

### Running the Server

To run the server, use the following command:

```sh
cargo run
```

The server will start at `http://localhost:8000`.

## API Endpoints

### Upload File

- **URL:** `/upload/<file_name>`
- **Method:** `POST`
- **Description:** Upload a file with the specified file name.
- **Example:**

  ```sh
  curl -X POST --data-binary @yourfile.txt http://localhost:8000/upload/yourfile.txt
  ```

### Download File

- **URL:** `/download/<file_name>`
- **Method:** `GET`
- **Description:** Download the file with the specified file name.
- **Example:**

  ```sh
  curl -O http://localhost:8000/download/yourfile.txt
  ```

### List Files

- **URL:** `/files`
- **Method:** `GET`
- **Description:** List all uploaded files.
- **Example:**

  ```sh
  curl http://localhost:8000/files
  ```

### Delete File

- **URL:** `/delete/<file_name>`
- **Method:** `DELETE`
- **Description:** Delete the file with the specified file name.
- **Example:**

  ```sh
  curl -X DELETE http://localhost:8000/delete/yourfile.txt
  ```

## Project Structure

```plaintext
.
├── Cargo.toml
├── src
│   └── main.rs
├── upload
│   └── (uploaded files will be stored here)
└── README.md
```

