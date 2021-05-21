# Optodice
Dice roller for Discord that uses your character from [Optolith](https://optolith.app)

With Optodice you can role Atribute and Skill checks on the values from your character.

**Attention:  
This software is still in an alpha state! The build from the Github Actions are only for test purposes!**

Optodice does not modify your characters, it operates in read only mode on your characters.

# Usage
Before you can use Optodice you have to install [Optolith Character Creator](https://optolith.app) and create or [import](https://12gem.me/uno-sguardo-nel-buio/raccolta-di-eroi/) a character.

After that you just need to know the following things before you can start Optodice.

## Discord Webhook 
Optodice requires a Discord Webhook URL to work.

### Creating a Discord Webhook:
To create a webhook for an Discord channel you need to have
the rights to do so. If you are not the owner/admin of the
channel, ask the administrator if he can create a webhook for
your party channel.
To create a webhook just right click the channel for which
you want to create a webhook. Then ***Edit Channel ->
Integrations -> Create Webhook***
You don't have to change Name and Avatar but be sure to
that the correct channel is selected. Then copy the webhook
URL and save it for late or send it to the 
other players. Optodice will ask you for this URL at the first start.

Example setup the developers use:
 - one voice/video channel for the gameplay
 - one channel for adventure/story related data (maps, portraits,
   names of characters, quest log, etc.)
 - one channel for Optodice output (check results, character status...)


## Avatar Upload Script (Optional but recomended)
The easiest way to use the avatar of the selected Hero with
the Discord Webhook is the Optodice avater upload script.

You must have a Webspace with PHP support.
Create a subfolder on your webspace and put the script into
this subfolder.

The script is called **upload-avatar.php** and can be found in the
[GitHub respository of Optodice](https://github.com/soulflyman/Optodice) in the folder ***php-avatar-script***.

### Example:
Your domain ***example.com*** points to your ***htdocs*** folder of
your webspace.
Then create a subfolder in ***htdocs*** called ***optodice*** and upload
the ***upload-avater.php*** script into this folder.
For this example, when Optodice asks you for your Upload-Script URL you would enter it like:
http://example.com/optodice/avatar-upload.php

If everything works correct, the avatar of the selected character
will be uploaded with the help of the upload script and is used
for every message Optodice sends to the Discord Webhook in
the name of your character.

## Role Checks
To role a Check on a attribute, skill or attack, you just have to klick the dice button in that corosponding row.
To Add some dificulty modification just add the number in the text box beside the dice button. Positive number (5, 2, +2, +1) lower the dificulty level, negative number (-1, -3) raise the dificulty level of that check.

## Character Status
To send the status of your character to Discord, just left click the avater. 

# Roadmap (some sort of) 
- [x] Select from multiple characters
- [x] Send data to a Discord Webhook
- [x] Role checks on attributes
- [x] Role checks on skills
- [x] Role checks on attacks
- [x] Display every check result in discord with the avatar of the character
- [x] Support automatic uploads of avatars (upload via php script workds, sftp upload is missing)
- [ ] Role custom dice/checks
- [x] Display avatar in Optodice
- [X] keep track of pain level
- [ ] keep track of money
- [x] Support Windows
- [x] Support Linux
- [ ] Support MacOS (github action builds are compiling, still not testet)
- [ ] i18n
- [ ] Settings view
- [ ] Options to autorole critical success/fails events
- [ ] Option to autorole battle damage
- [ ] Cumulative Group Checks Using Multiple Skills
- [ ] Save character state when switching characters or restarting the application
   
# Screenshots
Optodice UI  
![Screenshot of the Optodice UI](https://github.com/soulflyman/optodice/blob/main/.github/assets/screenshots/optodice.png?raw=true)

How it looks in Discord when you role a check  
![Screenshot of the Optodice messages in a Discord channel](https://github.com/soulflyman/optodice/blob/main/.github/assets/screenshots/discord.png?raw=true)