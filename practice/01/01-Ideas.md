# 01-Ideas
- [Youtube - Layout](https://youtu.be/tnILeOa_dto)
- [Youtube - Think & Sketch](https://youtu.be/AAIJumqYtns)
- [Youtube - Paint](https://youtu.be/iWwVZAEXyZM)
- [Github - PDF](https://github.com/FrankBevr/100-Ideas/blob/main/practice/01/01-Ideas.pdf)
- [Github - Code](https://github.com/FrankBevr/100-Ideas/blob/main/practice/01/state_creator_04/lib.rs)

<hr style="page-break-after: always;"/>

## Overview
### :wave: Intro 
- [x] I'm Frank.
### :brain: Think
- [x] Words 
- [x] Sentences 
- [x] Ideas
- [x] Rate Ideas
- [x] Select Idea
### :pencil: Sketch
- [x] Structure Idea (Design)
   - [x] Ideas (Problem/Solution)
   - [x] Requirments (Functional / NonFunctional)
   - [x] Stories
   - [x] Diagrams
       - [x] UseCase (What)
       - [x] Class (Data)
       - [x] Sequence (Behaviour)
### :art: Paint
- [x] First Draft
    - [x] `cargo contract new my_new_idea`
    - [x] `cargo contract build`
    - [x] `substrate-contracts-node --dev`
    - [x] `cargo contract deploy`
    - [x] `cargo contract call`
### :wave: Outro

- [x] **What we did:** Think, Sketch, Paint  
- [x] **How we achieved it:** Markdown, UML, ink!  
- [x] **Why:** ?Great Ideas come from many?  

<hr style="page-break-after: always;"/>

## :brain: Think
| Words              |                |                |           |                |
|:------------------ |:-------------- |:-------------- |:--------- |:-------------- |
| Giraffe            | Monkey         | City           | Tower     | Iot            |
| Augumented Reality | Vision Pro     | House          | Chair     | Female         |
| Male               | Lawyer         | Doctor         | Laptop    | Scientist      |
| Physicist          | Chemist        | Biologist      | Germanist | Ethistics      |
| Priest             | Coffee         | Latte Machiato | Shoes     | Poland         |
| Germany            | Europe         | China          | Chenzhen  | India          |
| State              | Candada        | Paint          | Color     | HSL            |
| Clothers           | Music          | Drop           | Frequency | Universe       |
| Family             | Children       | Toys           | ToyStory  | Animation      | 
| Weather            | Sad            | Shiny          | Sun       | Mercury        |
| Elements           | Plumbbumb      | Metal          | Oil       | Plastic        |
| Bell               | Manufacturuing | Vertictes      | Time      | Smile          |
| Band               | aShields       | Car            | Machines  | Packaging      |
| aFirehelper        | fusion         | Switzerland    | electron  | Collider       |
| Games              | Psychology     | Magazain       | Ocean     | Animals        |
| Whale              | Manga          | theories       | Books     | Amazon         |
| Netlify            | AWS            | Vercel         | Vim       | Blender        |
| Models             | Animation      | Uv Mapping     | Wheel     | Motion Capture |
| Terminal           | Plane          | Airport        | Raynair   | Memes          |
| Whatsapp           | Instagram      | Bumble         | Tinder    | Facebook       |


| Sentences                 | Rating | Description                                                                                          |
|:------------------------- |:------ |:---------------------------------------------------------------------------------------------------- |
| Animal tracking           | 5/10   | As a User I want to track my animals in my resovoir                                                  |
| City Funding              | 3/10   | As a City citizen I want to be able to fund projects in my city                                      |
| Data Distrubiton          | 4/10   | As a User I want to distrubte Data without loosing ownership                                         |
| Health Safe               | 6/10   | As a Doctor I want to have access to my Patients Healt Data                                          |
| Encyclopdia unstoppable   | 4/10   | As a Human I want to be access to none censored data                                                 |
| State Creator             | 8/10   | As a User I want to create my own empire.                                                            |
| My Color                  | 4/10   | As a User I want to have access to the lastest color trends                                          |
| Fashion Collector         | 2/10   | As a User I want to collect the latest Fashions Trends                                               |
| Music Distrubter          | 3/10   | As a Artist I want to distrubte my Music in a fair manner                                            |
| Elements Collector        | 8/10   | As a Scientist I want to learn about elements and have a proove that I know things                   |
| Weather Forecast          | 3/10   | As a Weather Scientist I want to share my forecast with the world without loosing my credentials     |
| Plastic Certificate       | 4/10   | As a Manufacture I want to certificate my new plastics                                               |
| Organic Snippet Collector | 8/10   | As a Biologist I want to share DNA Snippets annomyously without taking full ownership about the Data |
| Supply Chain Tracker      | 2/10   | As a Manufacatore I want to have a proof that my supply chain is child labor free                    |
| Animation Libary          | 4/10   | As a Animator I want to share my unique animation key frames with the world                          |

:::success
Selected Idea:  
:bulb: State Creator - Create your Empire
:::

<hr style="page-break-after: always;"/>

## :pencil: Sketch

### Idea

State Creator - Create your Empire

#### Problem
Its hard to create your own State

#### Solution
`statecreator.xyz`

### Requirments

#### Functional Requirments

- The App must do add citizens
- The App must allow elections
- The App must allow to buy land
- The App must allow festivals

#### NonFunctional Requirments

- The App must be able easy to use
- The App must be open source

### User Stories

#### Epic User
- As a User I want to become a citzien
- As a User I want to participate in elections

#### Epic State
- As a State I want to be able to buy land
- As a State I want to create festivals

### Diagrams

```plantuml
!theme plain
left to right direction
title Sketch - Use Case Diagram 
skinparam actorBorderThickness 1

actor Citizen
actor State

State -- (buy land)
State -- (do festival)

Citizen -- (become)
Citizen -- (participate)
```

</br>

```plantuml
!theme plain
left to right direction
title Sketch - Class Diagram
class Citizen{
    + name: String
    + id: Number
    + since: Date
}
class State {
    + Citizens: Citizen[]
    + Fesival: String
    
    + become_citizen()
    + do_festival()
    + buy_land()
}
State *-- Citizen
```

</br>

```plantuml
!theme plain
title Sketch - Sequence Diagram
actor Citizen
entity WebApp
database SmartContract
entity LandSeller

group Registration
WebApp --> SmartContract: listens
Citizen --> WebApp: wanna become citizen
WebApp --> SmartContract: add_member()
SmartContract --> SmartContract: checks()
WebApp --> Citizen: hurei you are member
end 

group FesivalCreation
SmartContract --> SmartContract: create_festival()
WebApp --> Citizen: festival starts
end

group LandBuy
Citizen --> WebApp: Lets buy this land
WebApp --> SmartContract: add_vote()
WebApp --> Citizen: Your vote was added
SmartContract --> LandSeller: give_me_land()
LandSeller --> SmartContract: sure, here, thanks
end
```

<hr style="page-break-after: always;"/>

## :art: Paint

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod state_creator_04 {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct StateCreator04 {
        value: bool,
        citizien: AccountId,
        festival: String,
    }

    impl StateCreator04 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: true,
                citizien: AccountId::from([0x0; 32]),
                festival: String::from(""),
            }
        }

        #[ink(message)]
        pub fn become_citizen(&mut self){
            self.citizien = self.env().caller();
        }

        #[ink(message)]
        pub fn get_citizen(&self) -> AccountId{
            self.citizien
        }

        #[ink(message)]
        pub fn do_festival(&mut self){
            self.festival = "20240324,Herbert,50".into();
        }

        #[ink(message)]
        pub fn get_festival(&mut self) -> String {
            self.festival.clone()
        }

        #[ink(message)]
        pub fn buy_land(& self){}

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