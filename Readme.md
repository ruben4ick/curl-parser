# Curl Parser
A parser that takes a curl command as plain text input, parses it, and extracts structured information from it.

## Technical description
It reads a single curl command and extracts the following request components:
- HTTP method (e.g., GET, POST)
- URL
- Headers
- Body

## Example input
```
curl -X POST "https://my.example.com/create" \
  -H "Authorization: Bearer 123" \
  -H "Content-Type: application/json" \
  -d '{"name": "New Item", "description": "It is a new item."}'
```

## Example output
```
method: POST
url: https://my.example.com/create
headers:
  Authorization: Bearer 123
  Content-Type: application/json
body:
{"name": "New Item", "description": "It is a new item."}
```