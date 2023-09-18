

# Features

| Feature  | Title                      | Description                                                                                                                  |
|----------|----------------------------|------------------------------------------------------------------------------------------------------------------------------|
| **1**    | **Car telemetry**          | Mostly, if not all, displayed on dash                                                                                        |
| 1.1      | - Speedometer              | Either via GPS or CanBus/OBD                                                                                                 |
| 1.2      | - Rev meter                | CanBus/OBD                                                                                                                   |
| 1.3      | - Engine temps             | CanBus/OBD                                                                                                                   |
| 1.4      | - Tachometer               | CanBus/OBD, Required by law                                                                                                  |
| 1.5      | - Trips                    | Some sort of automatic trip registration                                                                                     |
| 1.5.1    | -- Refueling               | keep track of Km's driven between refueling (simply trigger on (much) higher fuel level than previous level ("rising edge")) |
| 1.5.2    | -- Navigation              | Integrate with navigation to register actual Km's driven and fual used                                                       |
| 1.6      | - Fuel level               | CanBus/OBD                                                                                                                   |
| 1.7      | - Fuel economy             | To be combined with 1.5 (maybe use some API to register current fuel price when refueling?)                                  |
| **2**    | **Climate/fan controls**   | Temperature, fan speed, fan selector, AC, Window defrost                                                                     |
| 2.1      | - Digitized                |                                                                                                                              |
| 2.1.1    | -- Invisible               | Implement similar controls to https://www.youtube.com/watch?v=XVbuk3jizGM                                                    |
| 2.1.2    | -- Static                  | Some static UI on center display                                                                                             |
| 2.2      | - Physical                 | Fallback option in case 2.1 does not work out                                                                                |
| **3**    | **Navigation**             |                                                                                                                              |
| 3.1      | - display location         | Swipe on center display to move between dash and center                                                                      |
| 3.2      | - OpenStreetMap            | https://blogg.bekk.no/building-an-openstreetmap-app-in-rust-part-ii-933ca8d0c48a                                             |
| 3.4      | - Routing                  |                                                                                                                              |
| 3.3.1    | -- Saved routes            | Save locations like google maps does, but also store a preferred route (for example avoid the highway for my commute)        |
| 3.3.2    | -- Navigate to contact     | Use contact list information to navigate to selected contact                                                                 |
| 3.3.3    | -- Search / manual entry   | Just like google maps (also use 3.3.1 and 3.3.2 as sources for results)                                                      |
| 3.3.4    | -- Connectivity            | Android auto also does navigation I believe, maybe a shortcut / stepping stone                                               |
| **4**    | **Music**                  |                                                                                                                              |
| 4.1      | - Stream from phone        | This would presumably work with Android Auto/OpenAuto                                                                        |
| 4.2      | - Local                    | Store files on board                                                                                                         |
| 4.3      | - Custom spotify impl      | Last resort. Check https://github.com/Rigellute/spotify-tui for rust implementation                                          |
| **5**    | **Connectivity**           |                                                                                                                              |
| 5.1      | - Android auto             |                                                                                                                              |
| 5.1.1    | -- OpenAuto                | probably need to port to rust                                                                                                |
| 5.1.2    | -- Crankshaft              | Could take a look at and steal features I like                                                                               |
| 5.2      | - Apple CarPlay            | Apple ain't for me                                                                                                           |
| 5.3      | - Internet                 | This is likely a Must instead, foundational to other features                                                                |
| 5.3.1    | -- auto hotspot phone      | Still need to figure out how and what. GieltjE named some kind of configuration standard, ask name                           |
| 5.3.2    | -- SIM card slot on board? | Rather not, uncertain if/how it would work                                                                                   |
| 5.4      | - Bluetooth                | fallback in case OpenAuto doesn't pan out?                                                                                   |
| 5.5      | - GPS                      | There are GPS USB "dongles", usable?                                                                                         |

# MoSCoW

| Feature | Score  | Reason                                        |
|---------|--------|-----------------------------------------------|
| 1.1     | Must   | I want to persist current capabilities        |
| 1.2     | Must   | I want to persist current capabilities        |
| 1.3     | Must   | I want to persist current capabilities        |
| 1.4     | Must   | Required by law                               |
| 1.5.1   | Could  | Nice to have                                  |
| 1.5.2   | Could  | Nice to have                                  |
| 1.6     | Must   | I want to persist current capabilities        |
| 1.7     | Could  | Nice to have                                  |
| 2       | Must   |                                               |
| 2.1.1   | Should | Preferred but uncertain                       |
| 2.1.2   | Could  | Fallback of 2.1.1                             |
| 2.2     | Could  | Fallback of 2.1.1                             |
| 3.1     | Could  | Nice to have                                  |
| 3.2     | Must   | Major reason for starting this project        |
| 3.3     | Must   |                                               |
| 3.3.1   | Should | Really want this                              |
| 3.3.2   | Should | Really want this                              |
| 3.3.3   | Must   | Basic necessities                             |
| 3.3.4   | Should | Fallback for 3.3.1 - 3.3.3                    |
| 4       | Must   | Major reason for starting this project        |
| 4.1     | Should | Preferred                                     |
| 4.2     | Could  | Fallback for 4.1                              |
| 4.3     | Could  | Fallback for 4.2                              |
| 5.1.1   | Should | Want to have a strong interaction with phone  |
| 5.1.2   | Wont   | Build on top op OpenAuto                      |
| 5.2     | Wont   | I won't use it, PR's are welcome              |
| 5.3     | Must   | Foundational to other features                |
| 5.3.1   | Should | This should be the primary method             |
| 5.3.2   | Could  | Last resort if 5.3.1 totally fails            |
| 5.4     | Could  | I'd like to have BT working, but nice to have |
| 5.5     | Should | Prefer 5.1, but as fallback                   |

# Implementation

For now I use egui for UI's, thusfar seems to work pretty nice, there are still a couple of unknowns. 
Since I want fancy UI work with fades to transparent etc, I am not 100% sure egui is the right tool for this.
It does seem to have some options regarding 3d rendering, which I would like to use.

## Dash UI

### When car is stationary 
I'd like to show a 3d model of the car on the dash with states from car telemetry.
For example when the lights are on show both an icon and illuminate the lights on the model.
Or when a door is open, show it opened on the model
An acceptable fallback would be a 2d top down image

### When driving
Either automatically show navigation