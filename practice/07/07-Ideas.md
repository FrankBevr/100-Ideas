# 07-Ideas

## Overview

[TOC]

## :brain: Think

### 100 Words

Goal: Get richt pictures

- Kiddo: Blocks, Garden, Icescream, Bicycle, Football, Goalkeeper
- Bags: After Shopping Presentation, Kathrin, Gucci, Searching, Missing, 
- University: Furtunty, Physics, Mathematics, Marketing, Project Managment, Discreptance
- Iot: Smart Clothes, ESP22, Particle Photon, Humidity, Heartbeat, Temperature
- Kilt: Paris, Workshop, Dudley, Presentation, India, Identification, Certifcate, Heatlh
- ... 

:information_source: Always look for words which cover rich pictures.

### 10 Sentences
- Iot - Humidity - Climate Change - Keep your City Healthy - 5
- Kiddo - Goalkeeper - Football - Train your Up, Invest in uprising stars - 5
- Bags - Searching - Real Life Search - Track your Items- 3
- University - Discreptance - Dating - Aligmnetscoring - 2
- Kilt - Workshop - ... - 0

### Select & Rate

Selected: Citor - Keep your City Healthy

## :pencil: Sketch 

### Idea

Problem: Cities have a bad Climate
Idea: Citor - Keep your City Healthy
Solution: `Citor.xyz`

### Requirments

Functional Requirments
- The SmartContract must be able to controll right funding
- The User must Allow to post Sensor Data
- The GasSensor must be able to store Data 

Non Functional Reuqirments
- Cities must be attracked to it

### User Stories

- As a User I want to store my Sensor Data manually
- As a User I want to see which area is the worst
- As a User I want to see which area is the best
- As a GasSensor I want to post my data easily
- As a SmartContract I want to be able to route funding

### Diagrams

```plantuml
!theme plain
skinparam actorBorderThickness 1
left to right direction
title UseCase

User -- (store)
User -- (see good)
User -- (see bad)
GasSensor -- (post)
SmartContract -- (route)
```
</br>

```plantuml
!theme plain
title Class

class Citor {
    + area: distrcitcs[]
    + gasData: String 
    + store(_gasData: String)
    + get_gas_data(): String
    + add_district(_district: District)
    + is_good(_district: District): String
}
```

</br>

```plantuml
!theme plain
title Sequence

actor User
actor GasSensor
entity WebApp
database SmartContract


WebApp --> SmartContract: hoi, i listen to you


User -> WebApp: here some data, its great
WebApp -> SmartContract: safe that, mi amigo

GasSensor -> SmartContract: Here real time Data
SmartContract -> SmartContract: I safe it every 5 minutes

User -> WebApp: hey, is my Districtu great
WebApp -> User: yes, its super great

User -> WebApp: fund my district
WebApp -> SmartContract: hoi, he wants to fund it
SmartContract -> SmartContract: add money to district

User -> WebApp: let me build something cool
WebApp -> SmartContract: yeah sounds great, we have money
SmartContract -> User: here, money, do what you say gonna do


```

## :art: Paint

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod citor {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Citor {
        value: bool,
        area: String,
        gas_data: String,
    }

    impl Citor {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                area: String::from("Area52,"),
                gas_data: String::from("5,Good; "),
            }
        }

        #[ink(message)]
        pub fn get_area(&self) -> String {
            self.area.clone()
        }

        #[ink(message)]
        pub fn add_area(&mut self) {
            self.area.push_str(" Area 53,");
        }

        #[ink(message)]
        pub fn get_gas_data(&self) -> String {
            self.gas_data.clone()
        }

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

## Outro
- Think, Sketch, Paint
- Adaptable
- Potential

