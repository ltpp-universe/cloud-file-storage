<center>

## cloud-file-storage

[![](https://img.shields.io/crates/v/cloud-file-storage.svg)](https://crates.io/crates/cloud-file-storage)
[![](https://img.shields.io/crates/d/cloud-file-storage.svg)](https://img.shields.io/crates/d/cloud-file-storage.svg)
[![](https://docs.rs/cloud-file-storage/badge.svg)](https://docs.rs/cloud-file-storage)
[![](https://github.com/ltpp-universe/cloud-file-storage/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/cloud-file-storage/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/cloud-file-storage.svg)](./license)

</center>

[Official Documentation](https://docs.ltpp.vip/cloud-file-storage/)

[Api Docs](https://docs.rs/cloud-file-storage/latest/cloud_storage/)

> Cloud file storage server based on the Rust hyperlane framework, supporting multiple file types for upload.

## Using Existing Service (Slower Due to Multiple Server Relays)

> API URL: [https://file.ltpp.vip](https://file.ltpp.vip)

## Local Deployment

### Clone Repository

```sh
git clone git@github.com:ltpp-universe/cloud-file-storage.git
```

### Run the Server

```sh
cargo run
```

## API

### Upload File

#### Request Details

| Method | Path      | Query Parameter  | Request Body        | Description                                                    |
| ------ | --------- | ---------------- | ------------------- | -------------------------------------------------------------- |
| POST   | /add_file | key: `file_name` | Binary file content | Uploads a file. `file_name` should include the file extension. |

#### Response Format

| Field  | Type   | Description                                   |
| ------ | ------ | --------------------------------------------- |
| `code` | int    | Request status: 1 for success, 0 for failure. |
| `msg`  | string | Message describing the result.                |
| `data` | string | The URL of the uploaded file.                 |

### Example Responses

#### Success

```json
{
  "code": 1,
  "msg": "ok",
  "data": "https://file.ltpp.vip/aaaVaabOaabVaabTaabLaaaVaabWaabPaabJaab0aab1aabYaabLaabFaabIaabLaabKaaaVaabMaabPaabSaabLaaaVaaaYaaaWaaaYaaa1aaaVaaaWaaaYaaaVaaaWaaa1aaaVaabJaaa0aaaWaaa2aabIaaaXaaa0aabLaaa1aaa5aabKaabIaaa0aabLaabJaaa2aabJaaa1aabHaaa1aabHaaa0aaa4aaa5aabKaaaWaaaWaaaXaabKaabMaabJaabLaabHaabHaaa3aaa4aaa2aaa0aabHaabMaaa5aaaWaaaZaabHaabMaabHaabLaaa0aaa1aabLaabHaaa3aabHaabIaaa0aaa5aaaWaaaXaaa5aabIaaaWaaa3aaa3aabH.png"
}
```

#### Failure

```json
{
  "code": 0,
  "msg": "missing file_name",
  "data": ""
}
```

### Load File

> Use the URL returned by the upload API.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributions

Contributions are welcome! Please submit issues or pull requests.

## Contact

For any questions, please contact [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
