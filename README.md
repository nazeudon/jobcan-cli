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
#### Clone this repo
```
git clone git@github.com:nazeudon/jobcan-cli.git
```

#### Create Credential json file
```zsh
cd /path/to/this/repo
touch ./src/credential.json
vim ./src/credential.json
```

```vim
{
    "email": "youremail@mail.com",
    "password": "yourpassword"
}
```

#### Install
(If you haven't installed `cargo` yet.)
```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

We all need to
```zsh
cargo install
```

#### Now You Can Enjoy!
```zsh
jobcan start
jobcan end
```

## Todo
- credential.jsonをCLI作成できるよにする
- 工数をCLIから入力できるようにする