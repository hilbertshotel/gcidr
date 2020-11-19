# gcidr

grep cidr tool for terminal

## Usage

```
gcidr <pattern> <input file>
```
or
```
stdin | gcidr <pattern>
```

## Example

```
pattern:
35.208.0.0/16

input:
35.209.148.113
35.208.119.213
35.214.253.219
35.208.107.68
35.214.178.62
35.208.94.125
35.214.204.72
35.213.182.115
35.209.96.129
35.209.45.100

output:
35.208.119.213
35.208.107.68
35.208.94.125
```
