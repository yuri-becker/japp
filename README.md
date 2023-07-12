<a name="readme-top"></a>

[![Website](https://img.shields.io/website?label=japp.shuttleapp.rs&style=for-the-badge&url=https%3A%2F%2Fjapp.shuttleapp.rs)](https://japp.shuttleapp.rs)
[![GitHub Repo stars](https://img.shields.io/github/stars/yuri-becker/japp?style=for-the-badge)](https://github.com/yuri-becker/japp/stargazers)
[![AGPL License](https://img.shields.io/github/license/yuri-becker/japp?style=for-the-badge)](https://github.com/yuri-becker/japp/blob/main/LICENSE.txt)

<h1 align="center">
<img height="22" src="./web/public/assets/favicon/android-chrome-192x192.png" align="end"> JAPP

</h1>

<ol>
<li>
  <a href="#about-the-project">About The Project</a>
  <ul>
    <li><a href="#built-with">Built With</a></li>
  </ul>
</li>
<li>
  <a href="#development">Development</a>
  <ul>
    <li><a href="#prerequisites">Prerequisites</a></li>
    <li><a href="#usage">Usage</a></li>
  </ul>
</li>
<li>
    <a href="#documentation">Documentation</a>
  <ul>
    <li><a href="#entities">Entities</a></li>
  </ul>
</li>
</ol>

## About The Project

So at my employer we do [Planning Poker](https://en.wikipedia.org/wiki/Planning_poker) remotely
~~and doing it on Miro sucks~~ and I was unhappy with the current platform we use. Looking for other, free (as in freedom) alternatives
I found none that really fits our case, so I decided to do my own.

So JAPP is basically a way of doing Planning Poker.

Why should you do Planning Poker? Ask your managers or something I guess.

Why should you use JAPP? Well, if you really have to do Planning Poker (remotely), JAPP should be an okay solution.

### Built With

#### Frontend

[![TypeScript](https://img.shields.io/badge/TypeScript-20232A?style=for-the-badge&logo=typescript&logoColor=3178C6)](https://reactjs.org/)
[![pnpm](https://img.shields.io/badge/pnpm-20232A?style=for-the-badge&logo=pnpm&logoColor=F69220)](https://pnpm.io)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactjs.org/)
[![Vite](https://img.shields.io/badge/Vite-20232A?style=for-the-badge&logo=vite&logoColor=646CFF)](https://vitejs.dev)

... as well as [zustand](https://github.com/pmndrs/zustand), [jest](https://jestjs.io), [immer](https://immerjs.github.io/immer/) and
[wretch](https://github.com/elbywan/wretch).

#### Backend

[![Rust](https://img.shields.io/badge/Rust-20232A?style=for-the-badge&logo=rust&logoColor=FFFFFF)](https://www.rust-lang.org)
[![Rocket](https://img.shields.io/badge/Rocket-20232A?style=for-the-badge&logo=rust&logoColor=d33848)](https://rocket.rs)
[![MongoDB](https://img.shields.io/badge/MongoDB-20232A?style=for-the-badge&logo=mongodb&logoColor=47A248)](https://www.mongodb.com)

... and [okapi](https://crates.io/crates/okapi). Hosted on [shuttle](https://www.shuttle.rs)

~~the so-called MRRR stack~~

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Development

### Prerequisites

- [Node.js](https://nodejs.org/en/download/)
- [pnpm](https://pnpm.io/installation)
- [Rust & Cargo via rustup](https://www.rust-lang.org/tools/install)
- [A local MongoDB instance for development](https://www.mongodb.com/try/download/community)
- Recommendation - [IDEA-based IDE](https://www.jetbrains.com) or [vscode(ium)](https://vscodium.com) since project is already set up there

### Usage

### One-time setup

- Clone this repository and cd into it
- `pnpm --dir web install`

### Start

- `pnpm --dir web build:watch`
- `cargo shuttle run`

These commands are also pre-configured in vscode.

The build output of the web directory is served by the Rust server, so everything is available from the
server ([localhost:8000](http://localhost:8000) by default).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Documentation


### API

The server's API is auto-generated and available at [/api/swagger](http://localhost:8000/api/swagger) when the server is
started.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Events

When a client joins a Session (or creates one) it gets the current Session state.

Both the client and the server react to events happening within that Session (```SessionEvents```).
1. The server updates the database to reflect the new state.
2.  The clients connected to that sesion update its state. 

This approach has the great disadvantage that state updates have to be implemented twice - on the server and on the client. The client receiving the events enables animated state transitions which wouldnt be possible if the server simply told the client the new state.

In the future it might make sense to fully event-source the state and use a messaging broker.

#### Example case: Voting for a card

```mermaid
sequenceDiagram

actor Clients
participant Server
participant Database

Clients ->>+ Server: "I vote 5"
Note over Clients,Server: API call by a John's client
Server ->>+ Server: Run Command vote_card
par Server to Database
 Server ->> Database: Persist Session with updated Votes
and Server to Clients
 Server ->>+ Clients:  John voted 5
end
deactivate Server
Clients ->> Clients: Update state
deactivate Clients
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Entities

```mermaid
classDiagram

class Session {
 + ObjectId id ~~generated~~
 + string secret ~~generated~~ ~~encrypted~~
 + Participant[] participants
 + Issue[] issues
 + string[] scale
}

class Participant {
  + ObjectId id ~~generated~~
  + string? name
  + boolean? estimating
  + boolean? away
}

Session "1" *-- "*" Participant

class Issue {
  + ObjectId id ~~generated~~
  + string name
  + ObjectId startedBy ~~Participant~~
  + Round[] rounds
}

Session "1" *-- "*" Issue

class Round {
  + Vote[] votes
  + boolean revealed
  + ObjectId revealedBy ~~Participant~~
}

Issue "1" *-- "1..*" Round

class Vote {
  + ObjectId participant ~~Participant~~
  + number vote ~~scale~~
}

Round "1" *-- "*" Estimation

```


<p align="right">(<a href="#readme-top">back to top</a>)</p>
