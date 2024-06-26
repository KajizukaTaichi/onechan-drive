# OneChan Drive

OneChan Drive was born because below story.

- OneDrive causes auto-launched when started up PC.
- To hinder it, I renamed OneDrive's binary file name to `OneChanDrive.exe` with humor.
- I like that name. So I started this project.

## Usage
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
  curl -X POST --data-binary @hello.txt http://localhost:8000/upload/hello.txt
  ```

### Download File

- **URL:** `/download/<file_name>`
- **Method:** `GET`
- **Description:** Download the file with the specified file name.
- **Example:**

  ```sh
  curl -O http://localhost:8000/download/hello.txt
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
  curl -X DELETE http://localhost:8000/delete/hello.txt
  ```


