## 🎮 ProExt - an External Cheat for Counter-Strike 2
#### 📖 This repository is for educational purposes.
#### 📥 Want to get in touch? I'm on discord, **@Vytrol**.
#### ©️ This is an ***heavily modified*** rust implementation of **TKazer's** CS2_External project.
## 🎬 Showcase:
![showcase](./Showcase.png)
## 📋 Features:
- **ESP 👁️**
- **Aimbot 🎯**
- **Triggerbot 💨**
- **Crosshair 𐀏**
- **Radar 📡**
- **Bomb Timer 💣**
- **Spectator List 👀**
- **Styling 🖌️**
- ...and more to come!
## 💬 Q&A:
**🤔  Is this detectable?** <br>
**Yes, this can be detectable**, it is a user-mode cheat afterall. *It is important to note that you might not be detected. The cheat uses only RPM by default - with WPM features labeled under "Risky". Other factors such as suspicious gameplay also are important to consider.*<br><br>
**🤔  FPS drops when focusing on the menu, any way to fix that?** <br>
**Yes, there is an actual fix for this.** If you have turned on the developer console, you can simply run the following command to resolve this issue:<br><code>engine_no_focus_sleep 0</code><br><br>
**🤔  How can I get support for this software?** <br>
**Feel free to reach out to me personally**, if you have any questions or need help with something related to the software. *My discord username is @Vytrol, I also have a Twitter under the same handle.*<br><br>
**🤔  My antivirus says "X", is this a virus?** <br>
**Of course not,** it wouldn't be the brightest of ideas to open source a virus - and you can always build the software yourself if you have concerns with the auto-update system. Why the antivirus flagged the software in the first place could be because of various factors, but likely as this is a cheat.
## 🐛 Known Bugs:
- **⚠️ ESP might not work as expected**, as some players might seem to "teleport" suddenly. I suspect that the data which is being read from the game is sometimes outdated. *Thankfully, this doesn't occur too often - and when it does, it is more times than not the players far away.*
- **⚠️ The FOV circle might flicker from two different sizes**, this happens mostly when a scope is un(equipped) or the scope's zoom is changed. *An easy workaround is to only enable the FOV circle to adjust your FOV, and then promptly disable it.*
- **⚠️ The Triggerbot might hold the mouse for too long ([#3](https://github.com/vytrol/ProExt/issues/3))**, this seems to happen more often on tap mode, but could also occur on hold mode. *This issue has so far only been replicated on one machine, and might not occur for you.*
- **❓ Something missing?** - If you find a bug, you may open an issue on the repository and it will be added here, the bug may also be fixed in a later update.