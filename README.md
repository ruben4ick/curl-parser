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

## Grammar
```
//A single space or tab character.
whitespace = _{ " " | "\t" }

// A backslash followed by optional whitespace and one or more newlines.
slash = _{ "\\" ~ whitespace* ~ NEWLINE+ }

// “whitespace super-set”: any mix of spaces, tabs, newlines, or slashes.
wss = _{ (whitespace | NEWLINE | slash)* }

// Text in single quotes.
single_quoted = _{ "'"  ~ (!"'"  ~ ANY)* ~ "'" }

// Text in double quotes.
double_quoted = _{ "\"" ~ ( "\\\"" | !"\"" ~ ANY )* ~ "\"" }

// Either single or double quoted text.
quoted = _{ double_quoted | single_quoted }

// Unquoted token without spaces.
bare_word = @{ (!(whitespace | NEWLINE) ~ ANY)+ }

// Quoted or unquoted value.
value = { quoted | bare_word }

// Non-whitespace characters.
none_ws = { (!whitespace ~ ANY)+ }

// Raw http or https URL.
url_plain = @{ "http" ~ "s"? ~ "://" ~ none_ws }

// Quoted or raw URL.
url = { single_quoted | double_quoted | url_plain }

// HTTP method flag (-X, --request).
method_flag = { ( "-X" | "--request" ) ~ whitespace+ ~ value }

// Header flag (-H, --header).
header_flag = { ( "-H" | "--header"  ) ~ whitespace+ ~ value }

// Data flag (-d, --data).
data_flag = { ( "-d" | "--data" ) ~ whitespace+ ~ value }

// Any supported flag.
option = _{ method_flag | header_flag | data_flag }

// Full curl command.
curl = { SOI
  ~ ( "#" ~ (!NEWLINE ~ ANY)* ~ NEWLINE )*
  ~ wss ~ "curl" ~ wss
  ~ (option ~ wss)* ~ url? ~ (wss ~ option ~ wss)*
  ~ wss ~ EOI
}
```