

# Features

| Feature | Title                      | Description                                                                                                                  |
|---------|----------------------------|------------------------------------------------------------------------------------------------------------------------------|
| **1**   | **Car telemetry**          | Mostly, if not all, displayed on dash                                                                                        |
| 1.1     | - Speedometer              | Either via GPS or CanBus/OBD                                                                                                 |
| 1.2     | - Rev meter                | CanBus/OBD                                                                                                                   |
| 1.3     | - Engine temps             | CanBus/OBD                                                                                                                   |
| 1.4     | - Tachometer               | CanBus/OBD, Required by law                                                                                                  |
| 1.5     | - Trips                    | Some sort of automatic trip registration                                                                                     |
| 1.5.1   | -- Refueling               | keep track of Km's driven between refueling (simply trigger on (much) higher fuel level than previous level ("rising edge")) |
| 1.5.2   | -- Navigation              | Integrate with navigation to register actual Km's driven and fual used                                                       |
| 1.6     | - Fuel level               | CanBus/OBD                                                                                                                   |
| 1.7     | - Fuel economy             | To be combined with 1.5 (maybe use some API to register current fuel price when refueling?)                                  |
| **2**   | **Climate/fan controls**   | Temperature, fan speed, fan selector, AC, Window defrost                                                                     |
| 2.1     | - Digitized                |                                                                                                                              |
| 2.1.1   | -- Invisible               | Implement similar controls to https://www.youtube.com/watch?v=XVbuk3jizGM                                                    |
| 2.1.2   | -- Static                  | Some static UI on center display                                                                                             |
| 2.2     | - Physical                 | Fallback option in case 2.1 does not work out                                                                                |
| **3**   | **Navigation**             |                                                                                                                              |
| 3.1     | - display location         | Swipe on center display to move between dash and center                                                                      |
| 3.2     | - OpenStreetMap            | https://blogg.bekk.no/building-an-openstreetmap-app-in-rust-part-ii-933ca8d0c48a                                             |
| **4**   | **Music**                  |                                                                                                                              |
| 4.1     | - Stream from phone        | This would presumably work with Android Auto/OpenAuto                                                                        |
| 4.2     | - Local                    | Store files on board                                                                                                         |
| 4.3     | - Custom spotify impl      | Last resort. Check https://github.com/Rigellute/spotify-tui for rust implementation                                          |
| **5**   | **Connectivity**           |                                                                                                                              |
| 5.1     | - Android auto             |                                                                                                                              |
| 5.1.1   | -- Should                  | Want to have a strong interaction with phone                                                                                 |
| 5.1.2   | -- Wont                    | Build on top op OpenAuto (could still take a look at and steal features I like)                                              |
| 5.2     | - Apple CarPlay            | Apple ain't for me                                                                                                           |
| 5.3     | - Internet                 | This is likely a Must instead, foundational to other features                                                                |
| 5.3.1   | -- auto hotspot phone      | Still need to figure out how and what. GieltjE named some kind of configuration standard, ask name                           |
| 5.3.2   | -- SIM card slot on board? | Rather not, uncertain if/how it would work                                                                                   |
| 5.4     | - Bluetooth                | fallback in case OpenAuto doesn't pan out?                                                                                   |
| 5.5     | - GPS                      | There are GPS USB "dongles", usable?                                                                                         |

4. Music
   * streamed from phone? (either bluetooth or OpenAuto?)
   * local?
   * Custom spotify implementation? (look at the spotify tui app)
5. Connectivity
   1. Android auto?
      1. OpenAuto (written in c++, with UI, last update 5 years ago)
         * port logic to rust?
         * Use as is?
      2. Crankshaft? (build on top of OpenAuto)
   2. Apple CarPlay
      * Nope, not it! (PR's welcome)
   3. Internet
      * auto hotspot phone
   4. Bluetooth 
      * fallback in case OpenAuto doesn't pan out?

# MoSCoW

| Feature | Score  | Reason                                                                          |
|---------|--------|---------------------------------------------------------------------------------|
| 1.1     | Must   | I want to persist current capabilities                                          |
| 1.2     | Must   | I want to persist current capabilities                                          |
| 1.3     | Must   | I want to persist current capabilities                                          |
| 1.4     | Must   | Required by law                                                                 |
| 1.5     | Could  | Nice to have                                                                    |
| 1.6     | Must   | I want to persist current capabilities                                          |
| 1.7     | Could  | Nice to have                                                                    |
| 2       | Should | Still need to decide on implementation                                          |
| 3.1     | Could  | Nice to have                                                                    |
| 3.2     | Must   | Major reason for starting this project                                          |
| 4       | Must   | Major reason for starting this project                                          |
| 5.1.1   | Should | Want to have a strong interaction with phone                                    |
| 5.1.2   | Wont   | Build on top op OpenAuto (could still take a look at and steal features I like) |
| 5.2     | Wont   | Apple aint for me                                                               |
| 5.3     | Should | This is likely a Must instead, foundational to other features                   |
| 5.4     | Could  | I'd like to have BT working, but nice to have                                   |

# Implementation
