# TO DO

- Add tests
  - torrent -> xml
  - torrent -> json
  - torrent -> yaml

- Reverse process
  - yaml -> torrent
  - json -> torrent
  - xml -> torrent

- Refactor
- Handle different types of torrent (single file, multiple files)
- Handle other fields

Fix:
- Find a way to call with:
  - input_path, (here), (json)      ✅
  - input_path, output_path, (json) ✅
  - input_path, output_path, type   ✅
  - input_path, type (here)         ❌
