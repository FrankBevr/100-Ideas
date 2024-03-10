# 05 Ideas

- [Youtube](https://youtu.be/ZwXQOF1Vc6E)
- [Github - PDF](https://github.com/FrankBevr/100-Ideas/blob/main/practice/05/05-Ideas.pdf)
- [Github - Code](https://github.com/FrankBevr/100-Ideas/blob/main/practice/05/ocp/lib.rs)

<hr style="page-break-after: always;"/>

## Overview

- Intro
- Think
  - [x] 100 Words
  - [x] 10 Sentences
  - [x] rate & select
- Sketch
  - [x] Idea
  - [x] Requirements
  - [x] Stories
  - [x] Diagrams
- Paint
  - [x] write
  - [x] build
  - [x] call
- Outro

<hr style="page-break-after: always;"/>

## :brain: Think

### 100 Words

| Theme          | Words        |            |             |            |                       |
| :------------- | :----------- | :--------- | :---------- | :--------- | :-------------------- |
| Kiddo          | teddyBear    | Indianer   | Kinderwagen | Dog        | Rabbit                |
| Ground School  | Reading      | Teacher    | Backery     | May        | Melting               |
| Middle School  | Puperty      | Status     | Homework    | LanParty   | Tequilla              |
| High School    | Women        | Ethics     | Physics     | Ecconomics | Alice                 |
| Apprenticeship | Shoes        | PVC        | Leather     | Gerbung    | cutting, Selecting    |
| University     | Biomechanics | Physics    | Mathematics | Happy Day  | Exams Food,           |
| Job            | 3D Printing  | Hololens   | Metal       | Laser      | Sculpting, Invesotrs, |
| Web            | Text         | Markdown   | Typescript  | Deno       | SQL, Components,      |
| Blockchain     | EthGlobal    | Governance | Pictures    | Robots     | enterntainment,       |

### 10 Sentences

| Word         | Idea               | Phrase                     | Rating |
| :----------- | :----------------- | :------------------------- | :----- |
| 3D Printing  | 1 click Production | 1 Click Production         | 6      |
| May          | Glasproduction     | recycle old make new       | 5      |
| LanParty     | Networking Game    | network on chain           | 2      |
| Women        | Dating             | Tokenize your game         | 5      |
| Biomechanics | Analyse            | Train your Ai              | 3      |
| SQL          | Database           | Datascruture Converter     | 3      |
| Rabbit       | Animal Care        | Care real, produce virtual | 2      |
| cutting      | Tooling            | How to hold your scissor   | 2      |

### Select & Rate

3D Printing - 1 Click Production - Your dreams in one click

<hr style="page-break-after: always;"/>

## :pencil: Sketch

### Idea

**Problem:** Producing a Product is Hard  
**Idea:** 1 Click Production - Your dreams in one click  
**Solution:** `oneclickproduction.xyz`

### Requirements

**Functional Requirements:**

- The Smart Contract must be able to offers of atomic services
- The App must be slice a product in smaller subtasks
- The User must be able to insert his product description
- The User must be able to receive the best production line

**Non Functional Requirements:**

- The App must align with the German Law
- The App must have a slick Design

### Stories

- As a Smart Contract I want to be able to make offer to other contracts
- As a User I want to get the best production line for my product
- As a User I want to type only one description in.
- As a User I want to select different production lines

### Diagrams

```plantuml
!theme plain
skinparam actorBorderThickness 1
left to right direction
title UseCase

SmartContract -- (make offers)
User -- (get best)
User -- (inputtyping)
User -- (select)
```

</br>

```plantuml
!theme plain
title Class

class OCP {
    +input: String
    +offer: String
    +make_offers()
    +get_best()
}
```

</br>

```plantuml
!theme plain
title Sequence

actor User
entity Frontend
entity Backend
database SmartContract
database SmartContractServiceA

group asking
User -> Frontend: I want to produce a rock
end

group process
Frontend -> Backend: He wants to produce a rock
Backend -> Backend: refine_production_input()
Backend -> Backend: atomic_slicing()
Backend -> SmartContract: here some atomic tasks
SmartContract -> SmartContractServiceA: can you do rocking?
SmartContractServiceA -> SmartContract: sure, cost 3 Euro
SmartContract -> SmartContractServiceA: can you do packaging?
SmartContractServiceA -> SmartContract: sure, cost 1 Euro
SmartContract -> SmartContract: calculate_total()
SmartContract -> Backend: Thats what i got so far
Backend -> Frontend: Look thats what I have
end

group selection
Frontend -> User: Thats the best, but there are more options
User -> Frontend: I take the best, i trust you
Frontend -> SmartContract: he agrees
end

group sign
User -> SmartContract: signs
SmartContract -> SmartContractServiceA: now do your thingy
end

... 5 days passed ...

group feedback
Frontend -> User: Do you liked it?
User -> Frontend: Yeah, its amazing
end
```

<hr style="page-break-after: always;"/>

## :art: Paint

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ocp {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Ocp {
        value: bool,
        input: String,
        offer: String,
    }

    impl Ocp {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                input: String::from("I want 5 rocks"),
                offer: String::from("No offer mi amigo")
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn make_offers(&mut self) {
            self.offer = String::from("mining,5euro;packaging,3euro")
        }

        #[ink(message)]
        pub fn get_best(&mut self) -> String {
            self.offer.clone()
        }
    }
}
```

<hr style="page-break-after: always;"/>

## Outro

- Think, Sketch, Paint
- Adaptive
- Potential
