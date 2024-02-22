# 08-Ideas

## Overview
- Intro
- Think
    - 100 Words
    - 10 Sentences
    - Select & Rate
- Sketch
    - Idea
    - Reuqirments 
    - Stories
    - Diagram
- Paint
    - cargo contract new our_great_idea
    - cargo contract build
    - cargo contract call 
- Outro

## :brain: Think

### 100 Words

- Feburary, grey, cold, karneval, daffodils, 
- Home, Bred, Dog, Sister, Couch, TV
- Series, Sherlock, One Piece, Scrubs, Lie
- Street, yew gum, 
- Drinking, meters

## 10 Sentence 

- daffodils : Flowerbombing - Spread your seed - 7
- Sherlock: Analyso - Fraud Decetion  - 5
- Dog: Disabilties - Your Assitent on your feet - 4

## Rate & Select

`Daffodils` - Flowerbombing - Spread your seed

## :pencil: Sketch

### Idea

Problem: Too much grey in City
Idea: Flowerbombing - Spread your seed
Solution: `daffodils.xyz`

### Requirments

Functional Requirments:
- The App must do seed the city
- The User must able to select seeding area
- The User must be able to propose seed
- The Smart Contract must handle the funding part

NonFunction Requirments:
- The App should be in align with the England Law
- The App should be easy to use

### User Stories

- As a User I want to seed my city
- As a User I want to select where to seed
- As a User I want to propose seed
- As a Smart Contract I want to fund it nicely

### Diagrams

```plantuml
!theme plain
skinparam actorBorderThickness 1
left to right direction
title Use Case


User -- (seed)
User -- (select)
User -- (propose)
SmartContract -- (fund)
```

</br>

```plantuml
!theme plain

class daffodils{
     +seeds: Seed[]
     +proposals: Proposal[]
     +areas: Area[]
     +do_seed(_seed: Seed)
     +did_seed(): bool
     +fund()
     +propose(_proposal: Propsosal)
     +get_proposals() -> Proposal
}
```

</br>

```plantuml
!theme plain
title Sequence

actor User
entity WebApp
database SmartContract

group Seed City
User -> WebApp: I want to seed there
WebApp -> SmartContract: She wants to Seed there
SmartContract -> SmartContract: lets seed it
SmartContract -> SmartContract: check and then fund
WebApp -> User: Your seed and area is funded
end

group Propose new seed
User -> WebApp: I want this seed
WebApp -> SmartContract: She wants this seed
SmartContract --> SmartContract: add new seed to proposals
end

```

</br>

## :art: Paint

