A lightweight VPN client built with Rust and Tauri, optimized for personal WireGuard management. It streamlines the setup process by automatically SSH-ing into your servers to generate and configure keys for both client and server, turning your infrastructure into ready-to-use nodes.

---

* [Jump to Installation Steps](#installation-windows)
* [View TODO List](#todo)

![disconnected](https://github.com/user-attachments/assets/22e244a2-eba1-4674-afc9-57207f20c943)
![connected](https://github.com/user-attachments/assets/2f455fe2-baa6-4f0f-8018-0325d189fc4d)
![connected-side-panel](https://github.com/user-attachments/assets/5dfbbfbb-4a22-449b-abab-3c8ca4251c74)

### Installation (Windows)
1. Go to the [Releases](https://github.com/oscarbrehier/vpn/releases) page.
2. Download the latest `setup.exe` or `.msi` file.
3. Run the installer and follow the on-screen instructions.
4. **Note:** This application requires **Administrator Privileges** to manage network interfaces. You will be prompted with a UAC (User Account Control) dialog every time you open the app.

### TODO
- [ ] Add macOS Support (admin privileges)
- [ ] Add Linux Support (admin privileges)
- [ ] Auto-connect on untrusted Wi-Fi
- [ ] Kill-Switch
