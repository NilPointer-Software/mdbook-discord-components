# mdbook-discord-components

A mdBook preprocessor for building [Discord Webcomponents](https://github.com/skyra-project/discord-components)

This project is build on top of [skyra-project/discord-components](https://github.com/skyra-project/discord-components)

Work in progress

Test site: https://nilpointer-software.github.io/mdbook-discord-components/

## Yaml parser example

``` yaml
\``` discord yaml
- username: Spen
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
    Red: red
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
  attachments:
  - url: https://avatars.githubusercontent.com/u/63750675?s=100
  - url: https://avatars.githubusercontent.com/u/63750675?s=200
    width: 200
    height: 200
    alt: bigger
  components: # Component syntax is a bit funky for now
    - # This is an action row
      - type: success
        label: First row
    -
      - type: success
        label: Second row
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
- type: join
  content: | # You can put html tags inside yaml strings
    Welcome, <i style="color: a155ab;">Snazzah</i>. We hope you brought pizza.
- username: Snazzah
  content: No.
\```
```

Please look at the [yaml_parser.rs](mdbook-discord-components/src/parsers/yaml_parser.rs) file for available fields.

## YAML Model

### 1. System Message

| Field         | Type              | Description 
|---------------|-------------------|-------------
| type          | [SystemMessageType](#11-systemmessagetype) | The type of the system message
| channel_name? | Boolean           | Whether this message is to show channel name changes, used to match Discord's style 
| timestamp?    | String            | Text that will show where the message timestamp is
| content       | String            | The text of the system message

#### 1.1. SystemMessageType

SystemMessageType is a String with the following valid values:

| Value       | Description
|-------------|-------------
| alert       | System alert message
| boost       | Server boost message
| call        | Call message
| edit        | Channel edit message
| error       | System error message
| join        | Server join message
| leave       | Server leave message
| missed_call | Missed call message
| pin         | Pin message
| thread      | Thread message

### 2. Message

| Field        | Type                | Description 
|--------------|---------------------|-------------
| user_id?     | Snowflake           | Author user ID. Works only with a proper deployment
| username     | String              | The author username. Will overwrite data from user_id
| avatar?      | String              | The author avatar url. Will overwrite data from user_id
| color?       | String              | CSS valid color of the author username (analog to role color)
| timestamp?   | String              | Text that will show where the message timestamp is
| bot?         | Boolean             | Whatever the user is a bot. Will overwrite data from user_id
| verified?    | Boolean             | Whatever the bot is verified
| edited?      | Boolean             | If the message was edited
| ephemeral?   | Boolean             | If the message is ephemeral
| roles?       | String -> Color map | Role color map used to properly color role mentions
| embed?       | [Embed](#3-embed)   | A single embed element. If `embeds` is present, this will be the first embed show
| embeds?      | Array of [Embed](#3-embed) | Array of embed elements
| reactions?   | Array of [Reaction](#4-reaction) | Array of reactions
| attachments? | Array of [Attachment](#5-attachment) | Array of image attachments
| components?  | Array of [ActionRow](#6-actionrow) | Array of action rows
| invites?     | Array of [Invite](#8-invite) | Array of invitations
| content      | String              | The message content

### 3. Embed

| Field        | Type           | Description 
|--------------|----------------|-------------
| title?       | String         | Embed title text
| url?         | String         | Embed title url
| color?       | String         | CSS valid color of the embed
| description? | String         | Embed description text
| image?       | String         | Image url
| thumbnail?   | String         | Thumbnail url
| author?      | [Author](#31-author) | Embed author data
| fields?      | Array of [Field](#32-field) | Embed fields
| footer?      | [Footer](#33-footer)         | Embed footer

#### 3.1. Author

| Field  | Type   | Description 
|--------|--------|-------------
| text   | String | Embed author text
| image? | String | Author image url
| url?   | String | Embed author url

#### 3.2. Field

| Field         | Type    | Description 
|---------------|---------|-------------
| name          | String  | Field name
| value         | String  | Field value
| inline?       | Boolean | If the field should be inline. Requires `inline_index`
| inline_index? | Integer | The index of the field (position)

#### 3.3. Footer

| Field      | Type   | Description 
|------------|--------|-------------
| text?      | String | Footer text
| image?     | String | Footer image url
| timestamp? | String | Footer timestamp. Must be in the following format `01/31/2000`

### 4. Reaction

| Field        | Type    | Description 
|--------------|---------|-------------
| emoji        | String  | Emoji image url
| name?        | String  | The name of the reaction. Used as alternative text
| count?       | Integer | Reaction count. Must be positive
| interactive? | Boolean | If the reaction should be interactive
| reacted?     | Boolean | Should the reaction show up as reacted

### 5. Attachment

| Field   | Type    | Description 
|---------|---------|-------------
| url     | String  | The image attachment url
| width?  | Integer | The width of the image
| height? | Integer | The hight of the image
| alt?    | String  | The alternative text of the image

### 6. ActionRow

ActionRow is an Array of [Button](#7-button)

Example:
```
- username: Example
  content: This is an example of components
  components: # Component syntax is a bit funky for now
    - # This is an action row
      - type: success
        label: First row
    -
      - type: success
        label: Second row
```

### 7. Button

| Field        | Type       | Description 
|--------------|------------|-------------
| type         | [ButtonType](#71-buttontype) | The type of the button
| label        | String     | Button text
| disabled?    | Boolean    | Whatever the button should be disabled
| emoji?       | String     | Emoji image url
| emmoji_name? | String     | The name of the emoji
| url?         | String     | The url of the button if used with the `secondary` type

#### 7.1. ButtonType

ButtonType is a String with the following valid values:

| Value       | Description 
|-------------|-------------
| primary     | A blue button style
| secondary   | A grey button style
| success     | A green button style
| destructive | A red button style

### 8. Invite

| Field      | Type    | Description
|------------|---------|-------------
| name       | String  | Invite server name
| members    | Integer | Server member count. Must be positive
| online     | Integer | Currently online member count. Must be positive
| icon?      | String  | Server icon url
| partnered? | Boolean | Is the server partnered
| verified?  | Boolean | Is the server verified
