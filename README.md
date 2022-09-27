<a name="readme-top"></a>

[![Website](https://img.shields.io/website?label=japp.yuri.li&style=for-the-badge&url=https%3A%2F%2Fjapp.yuri.li)](https://japp.yuri.li)
[![GitHub Repo stars](https://img.shields.io/github/stars/yuri-becker/japp?style=for-the-badge)](https://github.com/yuri-becker/japp/stargazers)
[![AGPL License](https://img.shields.io/github/license/yuri-becker/japp?style=for-the-badge)](https://github.com/yuri-becker/japp/blob/main/LICENSE.txt)

<h1 align="center">JAPP</h1>

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
~~and doing it on Miro sucks~~ and I was unhappy with the current platform we use. Looking for other, free alternatives
I found none that really fits our case, so I decided to do my own.

So JAPP is basically a way of doing Planning Poker.

Why should you do Planning Poker? Ask your managers or something I guess.

Why should you use JAPP? Well, if you really have to do Planning Poker (remotely), JAPP should be an okay solution.

### Built With

[![TypeScript](https://img.shields.io/badge/TypeScript-20232A?style=for-the-badge&logo=typescript&logoColor=3178C6)](https://reactjs.org/)
[![Yarn](https://img.shields.io/badge/Yarn-20232A?style=for-the-badge&logo=yarn&logoColor=2C8EBB)](https://yarnpkg.com)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactjs.org/)

... as well as [zustand](https://github.com/pmndrs/zustand), [jest](https://jestjs.io) and
[wretch](https://github.com/elbywan/wretch).


[![Rust + Rocket](https://img.shields.io/badge/Rust-20232A?style=for-the-badge&logo=rust&logoColor=FFFFFF)](https://www.rust-lang.org)
[![Rocket](https://img.shields.io/badge/Rocket-20232A?style=for-the-badge&logo=rust&logoColor=d33848)](https://rocket.rs)
[![MongoDB](https://img.shields.io/badge/MongoDB-20232A?style=for-the-badge&logo=mongodb&logoColor=47A248)](https://www.mongodb.com)

... and [okapi](https://crates.io/crates/okapi).

~~the so-called MRRR stack~~

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Development

### Prerequisites

* [Node.js](https://nodejs.org/en/download/)
* [Yarn](https://yarnpkg.com/getting-started/install)
* [Rust & Cargo via rustup](https://www.rust-lang.org/tools/install)
* [MongoDB (Preferably local instance)](https://www.mongodb.com/try/download/community)
* [Google Cloud Command Line Tools (Only for Deployment)](https://cloud.google.com/sdk)
* [Recommendation - IDEA-based IDE since project is already set up there](https://www.jetbrains.com)

### Usage

### One-time setup

* Clone this repository and cd into it
* `yarn --cwd web install`

### Start

* `yarn --cwd web build:watch`
* `cargo run`.
    * During the startup, Rocket will probably complain about missing configurations.<br/>
      The easiest way to set these is via environment variables: e.g. `mongo_uri` is missing - set `ROCKET_MONGO_URL`.
    * You'll also want to set `ROCKET_SECRET` so that you don't lose your cookies when the server restarts.

These commands are also pre-configured in IntelliJ.

The build output of the web directory is served by the Rust server, so everything is available from the
server ([localhost:8000](http://localhost:8000) by default).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Documentation

Yea, I didn't document a lot, so I didn't want to make another file for this stuff.

### API

The server's API is auto-generated and available at [/api/swagger](http://localhost:8000/api/swagger) when the server is
started.

### Entities

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

<p align="right">(<a href="#readme-top">back to top</a>)</p>

