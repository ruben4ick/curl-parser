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
curl -X POST "https://example.com/do?param=value
"
-H "Authorization: Bearer some_token"
-H "Content-Type: application/json"
-d '{"amount":100,"email":"ao.ruban@ukma.edu.ua"}'
```