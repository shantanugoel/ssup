# sup!

`sup` is a simple cross-platform tool to send notifications via cli. It also supports an option to optionally run a given command and then send the notification once the command finishes execution.

# Currently supported platforms
- macOS
- Linux
- Windows

# Currently supported architectures
- x86_64

## Currently supported notification targets
- local (i.e. System notifications, this is provided by the great `notifica` crate)
- telegram

## CLI Options
```
20:53:15 ❯ ./sup -h
sup 0.2.0
Shantanu Goel <shantanu+sup@shantanugoel.com>
A utility to send notifications to local system or telegram

USAGE:
sup [OPTIONS] --message <message> [command]

ARGS:
<command>    Specify a command/executable to run and notify when it finishes

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-t, --title <title>                Optional title for the notification [default: Sup!]
-m, --message <message>            Notify with this message
-d, --destination <destination>    [default: local] [possible values: local, telegram]
    --chat-id <chat-id>            {Telegram specific option} Chat id to which notification should be sent
    --bot-token <bot-token>        {Telegram specific option } Optionally specify Telegram bot token in command instead of reading from env
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