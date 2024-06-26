# OneChan Drive

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

