# friendgrow

`friendgrow` is a command line tool to help you keep track of when to next see each of your friends, however close you are to them, making you a better friend who remembers everyone. Inspired by [Kunal's friendlog](https://github.com/marwahaha/friendlog).

## Introduction

### Install
- Download the latest binary using `cargo install friendgrow`
- Choose a database location and set that in your environment in your preferred way, e.g. `export FRIENDGROW_DB=~/.friendgrow.db >> .bashrc`

### Add friends
```
% ./friendgrow add Gandolf "Middle Earth"
Gandolf (Middle Earth) every 10 weeks, not seen yet
% ./friendgrow add Sam "The Shire" -f 3
Sam (The Shire) every 3 weeks, not seen yet
```

### Record seeing friends
```
% ./friendgrow record Gandolf 2021-10-06
Gandolf (Middle Earth) every 10 weeks, last seen on 2021-10-06, see next 2 days ago
% ./friendgrow record Sam 2021-12-2
Sam (The Shire) every 3 weeks, last seen on 2021-12-02, see next in 6 days
```

### Upcoming friends to see
```
% ./friendgrow upcoming
+---------+--------------+-----------+------------+------------+
| Name    | Location     | Frequency | Last seen  | Due        |
+---------+--------------+-----------+------------+------------+
| Gandolf | Middle Earth | 10 weeks  | 2021-10-06 | 2 days ago |
| Sam     | The Shire    | 3 weeks   | 2021-12-02 | in 6 days  |
+---------+--------------+-----------+------------+------------+
```

## Contribute
- Fork this repository
- Make sure you've installed rust and cargo
- Try out your local fork using `cargo run -- [friendgrow args]`
- To use a different test database file, temporarily set the location using `export FRIENDGROW_DB=./test-friendgrow.db`

### TODOs

Anyone can feel free to help with these!

- Add documentation comments
- When friend not found, suggest similar names from DB
