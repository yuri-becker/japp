# JAPP

## Domain

```puml
!theme vibrant
skinparam defaultFontSize 16

entity Session {
* id : string <<generated>>
--
* name : string
* secret : string <<encrypted | generated>>
* participants: Participant[]
* issues : Issue[]
* scale : String[] 
}

entity Participant {
* id : text <<generated>>
--
* name : text
* cookie : text <<encrypted | generated>>
estimating : boolean
away: boolean
}

Session *-- "n" Participant 

entity Issue {
* esimations: Estimation[]
* revealed : boolean
* startedBy: string (id of Participant)
name : string (only startedBy may set)
}
Session *-- "n" Issue

entity Estimation {
* participant : string (id of Participant)
* estimation : number (Index of Session::scale)
}
Issue *-- "n" Estimation
```
