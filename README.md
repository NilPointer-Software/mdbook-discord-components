# mdbook-discord-components

A mdBook preprocessor for building [Discord Webcomponents](https://github.com/skyra-project/discord-components)

Work in progress

Test site: https://nilpointer-software.github.io/mdbook-discord-components/

## Yaml parser example

``` yaml
\``` discord yaml
- username: Spen #696368083517964288
  color: "#b9a0e0"
  content: |
    !echo

- username: Wiki Bot
  color: "#b9a0e0"
  bot: true
  verified: true
  ephemeral: true
  edited: true
  timestamp: Today at 00:00
  reactions:
    - emoji: https://em-content.zobj.net/thumbs/120/mozilla/36/heavy-black-heart_2764.png
      name: ":heart:"
      count: 5
      interactive: true
      reacted: true
  roles:
    Blue: blue
  content: |
    Hello <@Spen> <@Blue> <#Channel>! Sent <t:8531 years ago>

    a
  embed:
    title: Test Embed
    timestamp: 07/07/2023
    description: |
      Hello embed!
    fields:
      - name: Test
        value: Hello
        inline: true
        inline_index: 1
      - name: Test 2
        value: aaa
        inline: true
        inline_index: 2
    footer:
      text: Hiii
\```
```

Please look at the [yaml_parser.rs](mdbook-discord-components/src/parsers/yaml_parser.rs) file for available fields.
