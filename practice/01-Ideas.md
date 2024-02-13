# 01-Ideas

- [24min- 01-Ideas Layout](https://youtu.be/tnILeOa_dto)
- [69min - 01-Ideas | Think & Sketch](https://youtu.be/jTiXy0bKQ-M)

## Overview
### Intro 
:ballot_box_with_check: I'm Frank.
### Think
1. :ballot_box_with_check: Words 
2. :ballot_box_with_check: Sentences 
3. :ballot_box_with_check: Ideas
4. :ballot_box_with_check: Rate Ideas
5. :ballot_box_with_check: Select Ideas
### Sketch
6. Structure Idea (Design)
   -  :ballot_box_with_check: Ideas (Problem/Solution)
   -  :ballot_box_with_check: Requirments (Functional / NonFunctional)
   -  :ballot_box_with_check: Stories
   - :ballot_box_with_check: Diagrams
       - :ballot_box_with_check: UseCase (What)
       - :ballot_box_with_check:Class (Data)
       - :ballot_box_with_check: Sequence (Behaviour)
### Paint
7. First Draft
    - `cargo contract new my_new_idea`
    - `cargo contract build`
    - `substrate-contracts-node --dev`
    - `cargo contract deploy`
    - `cargo contract call`
### Outro
What, How, Why

## Think
|Words|||||
|:-|:-|:-|:-|:-|
|Giraffe| Monkey| City| Tower| Iot|
|Augumented Reality| Vision Pro| House| Chair| Female|
|Male| Lawyer| Doctor| Laptop| Scientist|
|Physicist| Chemist| Biologist| Germanist| Ethistics|
|Priest| Coffee| Latte Machiato| Shoes| Poland|
|Germany| Europe| China| Chenzhen| India|
|State| Candada| Paint| Color| HSL|
|Clothers| Music| Drop |Frequency| Universe|
|Family| Children| Toys| ToyStory| Animation|
|Weather| Sad| Shiny| Sun| Mercury|
|Elements| Plumbbumb| Metal| Oil| Plastic|
|Bell| Manufacturuing| Vertictes| Time| Smile|
|Band| aShields| Car| Machines| Packaging|
|aFirehelper| fusion| Switzerland| electron| Collider|
|Games| Psychology| Magazain| Ocean| Animals|
|Whale| Manga| theories| Books| Amazon|
|Netlify| AWS| Vercel| Vim| Blender|
|Models| Animation| Uv Mapping| Wheel| Motion Capture|
|Terminal| Plane| Airport| Raynair| Memes| 
|Whatsapp| Instagram| Bumble| Tinder| Facebook| 

Sentences

1. Animal tracking - 5/10
2. City Funding - 3/10
3. Data Distrubiton - 4/10
4. Health Safe - 6/10
5. Encyclopdia unstoppable - 4/10
6. State Creator 8/10
7. My Color 4/10
8. Fashion Collector 2/10
9. Music Distrubter 3/10
10. Elements Collector 8/10
11. Weather Forecast 3/10
12. Plastic Certificate 4/10
13. Organic Snippet Collector 8/10
14. Supply Chain Tracker 2/10
15. Animation Libary 4/10

:::success
Selected Idea:  
:bulb: State Creator - Create your Empire
:::

## Sketch

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

#### UseCase

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

#### Class Diagram

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

#### Sequence Diagram

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