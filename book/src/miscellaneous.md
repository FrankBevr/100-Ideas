# Chapter 1

**Bold**

_Italic_

**_Bold & Italic_**

- Bullet point

1. ordered list
2. ordered list
3. ordered list

:grin: - No similey

`inline code`

```ts
console.log("This is typescript");
const a: number = 5;
```

```rust
fn main(){
  let a = 100
  println!("{:?}", a);
}
```

| My table    | Column A         | Column B |
| :---------- | :--------------- | :------- |
| Great Table | Wonderful Column | Oh wow   |

> My blockquote

[Linko]()

![Picturo](https://imgs.search.brave.com/dvJ-bx1jLZ0XdVBVLqfGwsfdbvaKea-RAdbsQ0hBeG4/rs:fit:860:0:0/g:ce/aHR0cHM6Ly93LmZv/cmZ1bi5jb20vZmV0/Y2gvNmEvNmEzMWI5/Yjg4MWM1YTlhMzQz/YjkyZTVlYmU4YzFh/NWYuanBlZw)

A citiation[^1]

[^1]: Does this work, yeap it does

aaand i forgot ~~strikethrough~~

- [ ] and tasklist
- [x] and tasklist

What is missing? plantuml

lets install it

discovered something

<div class="warning">

This is a bad thing that you should pay attention to.

Warning blocks should be used sparingly in documentation, to avoid "warning
fatigue," where people are trained to ignore them because they usually don't
matter for what they're doing.

</div>

A diagram in UTF-8 text format (inlined automatically)

```plantuml, format=png
@startuml
!include https://raw.githubusercontent.com/bschwarz/puml-themes/master/themes/cyborg-outline/puml-theme-cyborg-outline.puml


class MyGreatClass {
+ abc
+ def
- aprivateFunction()
+ aPublicFunction()
}

@enduml
```

Force png format:

```plantuml
@startuml
!include https://raw.githubusercontent.com/bschwarz/puml-themes/master/themes/cyborg-outline/puml-theme-cyborg-outline.puml
left to right direction

hello -- (talk)
@enduml
```

```plantuml
@startuml
!include https://raw.githubusercontent.com/bschwarz/puml-themes/master/themes/cyborg-outline/puml-theme-cyborg-outline.puml

actor User
entity WebApp
database SmartContract

User --> WebApp: do clicky
WebApp -> SmartContract: do calling
SmartContract -> User: do sendy money
@enduml
```
