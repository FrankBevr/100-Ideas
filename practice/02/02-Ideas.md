# 02-Ideas

- [Youtube](https://youtu.be/P_xxstTo5zY)
- [Github - PDF](https://github.com/FrankBevr/100-Ideas/blob/main/practice/02/02-Ideas.pdf)
- [Github - Code](https://github.com/FrankBevr/100-Ideas/blob/main/practice/02/researcho/lib.rs)
<hr style="page-break-after:always;"/>

## Overview

### :wave: Intro

- [x] I'm Frank

### :brain: Think

- [x] Words
- [x] Sentences
- [x] Rate
- [x] Select

### :pencil: Sketch

- [x] Idea
  - [x] Problem
  - [x] Solution
- [x] Requirements
  - [x] Functional Requirements
  - [x] Non Functional Requirements
- [x] Stories
- [x] Diagrams
  - [x] Use Case
  - [x] Class
  - [x] Sequence

### :art: Paint

- [x] `cargo contract new our_great_idea`
- [x] `cargo contract build`
- [x] `cargo contract call`

### :wave: Outro

- [x] What: Think, Sketch, Paint
- [x] How: Markdown, UML, ink!
- [x] Why: Fast

<hr style="page-break-after:always;"/>

## :brain: Think

> :dart: Goal Write 100 Words

| Words          |                |                |               |               |
| :------------- | :------------- | :------------- | :------------ | :------------ |
| monkey         | health         | orthesis       | services      | craftsmens    |
| lawyer         | packaging      | wallpainting   | iot           | ar            |
| xr             | mr             | art            | 3d            | 2d            |
| krita          | blender        | figma          | uiux          | thailand      |
| city           | funding        | germany        | azerbajan     | kyrgystan     |
| politics       | state          | statesmen      | light         | physics       |
| chemisit       | bilogist       | germanist      | lingustic     | ethics        |
| religion       | school         | education      | chair         | fashion       |
| online shops   | jackets        | consulting     | lawyer        | games         |
| mmorpg         | spaceships     | rockets        | asteroids     | planets       |
| solarsystem    | cars           | tractors       | farming       | food          |
| banana         | dirt           | excavator      | neurofeedback | construction  |
| datastructures | youtube        | facebook       | whatsapp      | tinder        |
| bumble         | erotic         | internet       | history       | technology    |
| sitting        | walking        | sports         | athletics     | fitness       |
| rope jumping   | inline skating | skating        | music         | r&b           |
| rock           | metal          | electro        | electro swing | jazz          |
| piano          | instruments    | gitare         | eyes          | mouth         |
| ears           | senses         | conductor      | capacity      | newton        |
| joule          | gravity        | cpu            | memory        | personalities |
| sub0           | polkadot       | smart contract | xcm           | 2024          |

> :dart: Goal: Write 10 sentences & rate it

| Name               | Catchphrase                                    | Rating |
| :----------------- | :--------------------------------------------- | :----- |
| Craftsmen Tooling  | Track your productivity                        | 6      |
| Digital Production | Distrubte your creations                       | 2      |
| Corruption         | keep politicians accountable                   | 7      |
| Clothes            | your clothes your production your distribution | 4      |
| History            | real history safe for the history no screwnity | 5      |
| Universe           | own your new planet                            | 3      |
| Chemistry          | fund the unknown                               | 7      |
| Sports             | get fair training                              | 2      |
| Instruments        | you voice your song                            | 2      |
| Hardware           | opensource next level                          | 4      |
| Calender           | time can be different                          | 2      |

> :dart: Goal Select: Chemistry - Fund the unknown

<hr style="page-break-after: always;"/>

## :pencil: Sketch

### Idea

#### Problem

Fund research with unknown outcome is difficult

#### Solution

`research.xyz`

### Requirements

#### Functional Requirements

- The app must allow the User to login
- The contract must fund projects
- The contract should be apply for funding

#### Non Functional Requirements

- The smart contract must be accessible to everyone

### Stories

#### User Story

- As a User I want to be able to login
- As a User I want to overview the current funded projects

#### Institution Story

- As a Institution I want reliable funding proposals

### Diagrams

```plantuml
!theme plain
skinparam actorBorderThickness 1
left to right direction
title Use Case Diagram
skinparam titleFontSize 14
skinparam titleFontStyle normal

actor User
actor Institution

User -- (login)
User -- (overview)
Institution -- (get proposal)
```

</br>

```plantuml
!theme plain
left to right direction
skinparam titleFontSize 14
skinparam titleFontStyle normal
title Class Diagram

class Researcho {
    + proposal: Proposal
    + member: String
    + get_proposal(): Proposal
    + get_member(): String
    + send_proposal()
}

class Proposal {
    +name
    +id
    +description
}

Researcho *-- Proposal

```

</br>

```plantuml
!theme plain
title Sequence Diagram
skinparam titleFontSize 14
skinparam titleFontStyle normal
actor User
actor Institution
entity WebApp
database SmartContract

WebApp -> SmartContract: listens
User -> WebApp: let me in
WebApp -> SmartContract: become_member()
WebApp -> User: hurei, you are a member
SmartContract -> SmartContract: checks proposals
SmartContract -> Institution: send_proposal
Institution -> SmartContract: I take it its great
```

<hr style="page-break-after:always;"/>

## :art: Paint

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod researcho {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Researcho {
        value: bool,
        proposal: String,
        member: AccountId,
    }

    impl Researcho {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                proposal: String::from(""),
                member: AccountId::from([0x0;32])
            }
        }

        #[ink(message)]
        pub fn get_proposal(& self) -> String {
            self.proposal.clone()
        }

        #[ink(message)]
        pub fn become_member(&mut self){
            self.member = self.env().caller();
        }

        #[ink(message)]
        pub fn get_member(&mut self) -> AccountId{
            self.member
        }

        #[ink(message)]
        pub fn send_proposal(&self) {}

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}

```
