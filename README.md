# sup!

`sup` is a simple cross-platform tool to send notifications via cli. It also supports an option to optionally run a given command and then send the notification once the command finishes execution.

# Currently supported platforms
- macOS
- Linux
- Windows

# Currently supported architectures
- x86_64

## Currently supported notification targets
- local (i.e. System notifications)
- telegram

## CLI Options
```
~/dev/rust/sup/target/release main*
19:59:16 ❯ ./sup --help
sup
Shantanu Goel <shantanugoel.com>
Notifications should be sent to local system

USAGE:
sup [OPTIONS] <SUBCOMMAND>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-m, --message <message>    Notify with this message [default: Hey!]
-r, --run <run>            Specify an executable to run and notify when it finishes

SUBCOMMANDS:
local
telegram
help        Prints this message or the help of the given subcommand(s)

~/dev/rust/sup/target/release main*
19:59:25 ❯ ./sup help local
sup-local

USAGE:
sup local

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

~/dev/rust/sup/target/release main*
19:59:32 ❯ ./sup help telegram
sup-telegram

USAGE:
sup telegram [FLAGS] [OPTIONS] --tg-chat-id <tg-chat-id>

FLAGS:
-h, --help                Prints help information
--tg-bot-token-env    Take Telegram bot token from SUP_TG_BOT_TOKEN env variable
-V, --version             Prints version information

OPTIONS:
--tg-bot-token <tg-bot-token>    Specify Telegram bot token in command
--tg-chat-id <tg-chat-id>        Telegram chat id to which notification should be sent
```

## Telegram Usage
- Create a bot and get a bot token following [these instructions](https://core.telegram.org/bots#6-botfather)
- Send a message to your bot from the telegram account on which you want to receive notifications
- Get the chat id by running the below command
  - `curl https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/getUpdates | jq .message.chat.id`
  - Replace `$TELEGRAM_BOT_TOKEN` by the token value that you got in step 1
  - You may need to install `curl` and `jq` if you don't have them already
    - Alternatively, you can enter this url in your browser `https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/getUpdates` and search for `id:` to get the value
- Use the bot token and chat id values in the CLI options given above. 