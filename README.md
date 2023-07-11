# Maigacha
Maigacha is a simple command-line tool for pulling random items from a list with different probabilities based on common or rare rates.

## Usage
To add an item to the list, use the add command with the name and the pull chance:

```shell
$ maigacha add "Item 1" common 0.5
```

To pull a random item from the list, use the pull command:

```shell
$ maigacha pull
Pulled a Common
"Item 1" : 0.5
```

To view the list, use the list command:
```shell
$ maigacha list
-Common Pulls-
"Item 1" : 0.5
-Rare Pulls-
"Item 2" : 2
```

To remove an item from the list, use the remove command with the name:

```shell
$ maigacha remove "Item 1"
"Item 1", has been removed.
```

## Installation
```shell
$ cargo install --git https://github.com/nynaceae/maigacha.git
```
