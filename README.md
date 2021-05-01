# Optodice
Dice roller for Discord that uses your heroes from [Optolith](https://github.com/elyukai/optolith-client)

With Optodice you can role Atribute and Skill checks on the values from your Hero.

**Attention:  
This software is still in an alpha state! The build from the Github Actions are only for test purposes!**

Optodice does not modify your heroes, it operates in read only mode on your heroes.

## Some sort of a roadmap
- [x] Select from multiple heroes
- [x] Send data to a Discord Webhook
- [x] Role checks on attributes
- [x] Role checks on skills
- [x] Role checks on attacks
- [x] Display every check result in discord with the avatar of the hero
- [x] Support automatic uploads of avatars (upload via php script workds, ftp upload is missing)
- [ ] Role custom dice/checks
- [x] Display avatar in Optodice
- [ ] keep track of pain level
- [ ] keep track of money
- [x] Support Windows
- [x] Support Linux
- [ ] Support MacOS (github action builds are compiling, still not testet)
- [ ] i18n
- [ ] Settings view
- [ ] Options to autorole critical success/fails events
- [ ] Option to autorole battle damage
- [ ] Cumulative Group Checks Using Multiple Skills

## Screenshots
Optodice UI  
![Screenshot of the Optodice UI](https://github.com/soulflyman/optodice/blob/main/.github/assets/screenshots/optodice.png?raw=true)

How it looks in Discord when you role a check  
![Screenshot of the Optodice messages in a Discord channel](https://github.com/soulflyman/optodice/blob/main/.github/assets/screenshots/discord.png?raw=true)