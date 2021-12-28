# jobcan-cli
ジョブカンの出勤/退勤をCLIで行えます

## Usage
出勤
```zsh
jobcan start
```

退勤
```zsh
jobcan end
```

## Set up
### Clone this Repo
```
git clone git@github.com:nazeudon/jobcan-cli.git
```

### Install
(If you haven't installed `cargo` yet)
```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

We all need to
```zsh
cargo install --path .
```

### Register your JOBCAN Account
Follow the CLI to input
(You can run it from any directory of your choice)
```zsh
jobcan auth
```

### Now You Can Enjoy!
(You can run it from any directory of your choice)
```zsh
jobcan start
jobcan end
```

## Todo
- 工数をCLIから入力できるようにする