# stealer-i

#### Implementation of stealer-i in the process of intensive cybersecurity from ICC (Inha Cybersecurity Club).

---

## Features of the stealer-i
- screenshot of the desktop.
- information about the operating system.
- telegram's session.
- logger for tracking processes.
- stealing passwords from Wi-Fi networks.
- the program does not create files on the client side, which will help to avoid unnecessary loads on the victim's device.

## It's important to know
- the program was created solely for training purposes and was not used for malicious purposes.
- there is a server folder that is needed to start the server in order to receive data from the client.

## Server
To start the server to receive information, you need to run the code from the ```server/mod.rs``` module. The server requires a cargo package ```zip = "0.5.12"```.