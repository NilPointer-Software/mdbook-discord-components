# mdbook-discord-components

A mdBook preprocessor for building [Discord Webcomponents](https://github.com/skyra-project/discord-components)

Work in progress

Test site: https://nilpointer-software.github.io/mdbook-discord-components/

## Yaml parser example

``` yaml
\``` discord yaml
- username: Spen # Content, username (or user_id) are required for a message to be valid
  user_id: 696368083517964288 # This filed will only work on a proper mdbook-discord-components deployment
  color: "#b9a0e0"
  highight: true
  content: |
    !echo

- username: Wiki Bot
  avatar: https://avatars.githubusercontent.com/u/63750675
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
  roles: # Role map to properly color role mentions (RoleName: role_color)
    Blue: blue
  content: |
    Hello <@Spen> <@Blue> <#Channel>! Sent <t:8531 years ago>
  embed:
    title: Test Embed
    url: https://github.com/NilPointer-Software/mdbook-discord-components
    color: red
    timestamp: 07/07/2023
    description: Hello embed!!
    author:
      text: Author
      image: https://avatars.githubusercontent.com/u/63750675
      url: https://github.com/NilPointer-Software/mdbook-discord-components
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
  embeds:
  - title: Test Embed 2
    timestamp: 07/07/2023
    description: Hello embed!!
    image: https://avatars.githubusercontent.com/u/63750675?s=100
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
  - title: Test Embed 3
    timestamp: 07/07/2023
    description: Hello embed!!
    thumbnail: https://avatars.githubusercontent.com/u/63750675
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
