# ssup!

`ssup` is a simple cross-platform tool to send notifications via cli to local system or telegram.
It can also be used to get alerted after a long-running (or otherwise) command finishes. This can be used either by specifying the command at the end of the ssup command, or by chaining ssup with other commands. See the examples/CLI options below for more.

## Examples
```
# Send a notification to local system
$ ssup -m "Hello world!"

# Send a notification to telegram
$ export SSUP_TG_BOT_TOKEN=<Your telegram bot token>
$ ssup -m "Hello world!" -d telegram -c <your-telegram-chat-id> 

# Run a custom command and send notification after it finishes. Advantage of this is that ssup will also report whether command was successful or not.
$ ssup -m "Hello world!" sleep 5

# Alternate ways to run a custom command and send notification after it finishes
$ sleep 5; ssup -m "Hello world!"  # Always send notification
$ sleep 5 && ssup -m "Hello world!" # Send notification only on success
$ sleep 5 || ssup -m "Hello world!" # Send notification only on failure

```

# Currently supported platforms
- macOS
- Linux
- Windows

# Currently supported architectures
- x86_64
- aarch64 (e.g. Raspberry Pi 4b, Apple M1)

## Currently supported notification targets
- local (i.e. System notifications, this is provided by the great `notifica` crate)
- telegram

## CLI Options
```
  █████  █████ ████ ████████ 
 ███░░  ░░███ ░███ ░░███░░███
░░█████  ░███ ░███  ░███ ░███
 ░░░░███ ░███ ░███  ░███ ░███
 ██████  ░░████████ ░███████ 
░░░░░░    ░░░░░░░░  ░███░░░  
                    ░███     
                    █████    
                   ░░░░░     
ssup 0.2.1 - Shantanu Goel <shantanu+ssup@shantanugoel.com>
A utility to send notifications to local system or telegram

USAGE:
ssup [OPTIONS] --message <message> [command]

ARGS:
<command>    Specify a command/executable to run and notify when it finishes

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-t, --title <title>                Optional title for the notification [default: Ssup!]
-m, --message <message>            Notify with this message
-d, --destination <destination>    Choose where to send the notification to [default: local] [possible values: local, telegram]
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

# Building
For linux platforms, this crate has a native dependency on libdbus, so you'd need to install a corresponding package (e.g. libdbus-1-dev on debian/ubuntu) before trying to compile it.

# TODO
- [ ] Add support for 32b (aarch32 and i686/386) linux builds
- [ ] Add custom webhook destination
- [ ] Add tests
- [ ] Add documentation
- [ ] Package and publish to crates.io
- [ ] ?