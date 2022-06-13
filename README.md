# Welcome to the JitGuardian Password Manager! 🗝️

**JitGuardian is a lightweight password manager app for desktop. JitGuardian strives to teach it's users about encryption and cryptography while providing a secure platform for them to safely store all their valuable login information.**

# Why we chose a password manager:
Modern password managers are overcomplicated. Our aim is to make a simple, secure, and user friendly password manager that is accessible for everyone. Built from the ground up 🛠️ using Rust🔥, our program guarantees to be blazing fast 🚀, memory safe 🔒, all while providing top of the line security 🏅 with local password and login saves.

<img src="https://jitguardian.tech/assets/img/JitGuardian-SC.png" style="margin-left:auto;margin-right:auto;display:block;"></img>

# Team Members: 👦🏾👨🏾👨🏾👦🏾
#### Tahir Chaudhry
- **GitHub: [Raforawesome](https://github.com/Raforawesome)**
- **Discord: Raforawesome#6666 (168704403400949761)**
- **Email: raforawesome@raforaweso.me**

#### Rafi Miah
- **GitHub: [Ad0pt3d](https://github.com/Ad0pt3d)**
- **Discord: Ad0pt3d#1887 (272357521480482818)**
- **Email: rafimiah2005@gmail.com**

#### Shaurya Santhosh
- **GitHub: [shawarma-s](https://github.com/shawarma-s)**
- **Discord: shawarma#6969 (770017815801102367)**
- **Email: snt.shaurya@gmail.com**

#### Fayaaz Ahmed
- **GitHub: [FayaazA](https://github.com/FayaazA)**
- **Discord: Fayaaz#5395 (761733878934798359)**
- **Email: fayaaz.ahmed.20@gmail.com**

## Project Structure Breakdown
### src-tauri/
This directory contains the entire backend (Rust part) of the project.  This includes filesystem abstractions, the cryptography and encryption modules, the vault manager, etc.

### docs/
This directory contains the source for the website located at [our website](https://jitguardian.tech).

### src/
This directory contains the entire source for the Vue.JS frontend.

## How To Build:

**Ensure you have Rust and Node.JS installed.**

In the project directory, run:
```
npm install
```
to install all the dependencies. It might take a minute.

Then, run:
```
npm run tauri:serve
```
and wait for the build process to finish.  It can take several minutes for the first compile, but much less for every subsequent compile.
